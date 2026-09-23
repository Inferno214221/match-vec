use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, PatConst, PatLit, PatPath, PatRange, PatReference, PatRest, PatStruct, PatTuple, PatTupleStruct, PatWild};

use crate::{VecPat, VecPatIdent, VecPatSlice};

pub enum Catchall<'a> {
    Rest(&'a PatRest),
    Ident(&'a VecPatIdent),
}

impl VecPat {
    fn as_catchall(&self) -> Option<Catchall<'_>> {
        match self {
            VecPat::Ident(
                ident @ VecPatIdent { subpat: Some((_, subpat)), .. }
            ) if subpat.is_catchall() => {
                Some(Catchall::Ident(ident))
            },
            VecPat::Rest(rest) => Some(Catchall::Rest(rest)),
            _ => None,
        }
    }

    fn is_catchall(&self) -> bool {
        match self {
            VecPat::Ident(
                VecPatIdent { subpat: Some((_, subpat)), .. }
            ) => subpat.is_catchall(),
            VecPat::Rest(_) => true,
            _ => false,
        }
    }

    fn ident_pat(&self) -> Option<TokenStream> {
        match self {
            VecPat::Ident(VecPatIdent { attrs, by_ref, mutability, ident, .. }) => Some(quote! {
                #(#attrs)*
                let #by_ref #mutability #ident
            }),
            _ => None,
        }
    }
}

pub trait GenerateMatchBody {
    fn gen_match_body(&self, body: &Expr) -> TokenStream;
}

impl GenerateMatchBody for VecPat {
    fn gen_match_body(&self, body: &Expr) -> TokenStream {
        match self {
            VecPat::Const(const_)    => const_.gen_match_body(body),
            VecPat::Ident(ident)     => ident.gen_match_body(body),
            VecPat::Lit(lit)         => lit.gen_match_body(body),
            VecPat::Path(path)       => path.gen_match_body(body),
            VecPat::Range(range)     => range.gen_match_body(body),
            VecPat::Reference(ref_)  => ref_.gen_match_body(body),
            VecPat::Rest(est)        => est.gen_match_body(body),
            VecPat::Slice(slice)     => slice.gen_match_body(body),
            VecPat::Struct(struct_)  => struct_.gen_match_body(body),
            VecPat::Tuple(tup)       => tup.gen_match_body(body),
            VecPat::TupleStruct(tup) => tup.gen_match_body(body),
            VecPat::Wild(wild)       => wild.gen_match_body(body),
        }
    }
}

impl VecPatIdent {
    fn gen_binding(&self) -> TokenStream {
        let VecPatIdent { attrs, by_ref, mutability, ident, .. } = self;
        quote! {
            #(#attrs)*
            let #by_ref #mutability #ident = __match_vec_vec;
        }
    }
}

impl GenerateMatchBody for VecPatIdent {
    fn gen_match_body(&self, body: &Expr) -> TokenStream {
        let binding = self.gen_binding();
        quote! {
            #binding
            #body
        }
    }
}

impl GenerateMatchBody for VecPatSlice {
    fn gen_match_body(&self, body: &Expr) -> TokenStream {
        let mut elems = self.elems.iter().peekable();
        let mut first = Vec::new();

        while let Some(elem) = elems.peek() && !elem.is_catchall() {
            first.push(*elem);
            elems.next();
        }

        let mut catchall = None;
        let before;
        let mut after = Vec::new();

        if let Some(elem) = elems.peek() {
            catchall = Some(elem.as_catchall().unwrap_or_else(|| panic!("{:?}", quote!(#elem))));
            elems.next();

            before = first;
            for elem in elems {
                assert!(!elem.is_catchall(), "pattern may only contain one catchall element");
                after.push(elem);
            }
        } else {
            before = Vec::new();
            after = first;
        }

        let mem_drop = quote!(::core::mem::drop);

        let before_len = before.len();

        let len_block = match catchall {
            Some(_) => quote!(let __match_vec_rem_len = __match_vec_slice.len();),
            None    => quote!(),
        };

        let before_block = if before.is_empty() {
            quote!()
        } else {
            let mut bindings = TokenStream::default();
            for elem in before {
                bindings.extend(
                    match elem.ident_pat() {
                        Some(ident) => quote! {
                            #ident = unsafe { __match_vec_drain.next().unwrap_unchecked() };
                        },
                        None => quote! {
                            let _ = __match_vec_drain.next();
                        },
                    }
                );
            }

            quote! {
                let mut __match_vec_drain = __match_vec_vec.drain(..#before_len);
                #bindings
                #mem_drop(__match_vec_drain);
            }
        };

        let after_block = if after.is_empty() {
            quote!()
        } else {
            let mut bindings = TokenStream::default();
            for (i, elem) in after.iter().enumerate() {
                bindings.extend(
                    match elem.ident_pat() {
                        // TODO: Should implement a pop_n<T, N: usize>(vec: Vec<T>) so that unsafe code isn't macro generated.
                        // Needs imports from the final crate.
                        Some(ident) => quote! {
                            #ident = unsafe {
                                __match_vec_spare[#i].assume_init_read()
                            };
                        },
                        None => quote! {
                            unsafe { __match_vec_spare[#i].assume_init_drop() };
                        },
                    }
                );
            }

            let new_len_expr = match catchall {
                Some(_) => quote!(__match_vec_rem_len),
                None => quote!(0),
            };

            quote! {
                unsafe { __match_vec_vec.set_len(#new_len_expr) };
                let __match_vec_spare = __match_vec_vec.spare_capacity_mut();
                #bindings
            }
        };

        let bind_block = match catchall {
            Some(Catchall::Ident(ident)) => ident.gen_binding(),
            _ => quote!(),
        };

        quote! {
            #len_block
            #before_block
            #after_block
            #bind_block
            #body
        }
    }
}

macro_rules! gen_match_body_default {
    ($T:ty) => {
        impl GenerateMatchBody for $T {
            fn gen_match_body(&self, body: &Expr) -> TokenStream {
                quote!(#body)
            }
        }
    };
    ($T:ty, $($N:ty),+ $(,)?) => {
        gen_match_body_default!($T);
        gen_match_body_default!($($N),+);
    }
}

gen_match_body_default! {
    PatConst,
    PatLit,
    PatPath,
    PatRange,
    PatReference,
    PatRest,
    PatStruct,
    PatTuple,
    PatTupleStruct,
    PatWild,
}