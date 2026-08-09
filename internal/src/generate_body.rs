use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, Ident, PatConst, PatLit, PatPath, PatRange, PatReference, PatRest, PatStruct, PatTuple, PatTupleStruct, PatWild};

use crate::{VecPat, VecPatIdent, VecPatSlice};

pub enum Catchall<'a> {
    Rest(&'a PatRest),
    Ident(&'a Ident),
}

impl VecPat {
    fn as_catchall(&self) -> Option<Catchall<'_>> {
        match self {
            VecPat::Ident(
                VecPatIdent { ident, subpat: Some((_, subpat)), .. }
            ) => if subpat.is_catchall() {
                Some(Catchall::Ident(ident))
            } else {
                None
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

    fn ident(&self) -> Option<&Ident> {
        match self {
            VecPat::Ident(ident) => Some(&ident.ident),
            _ => None,
        }
    }
}

pub trait GenerateMatchBody {
    fn gen_match_body(&self, vec: &Expr, body: &Expr) -> TokenStream;
}

impl GenerateMatchBody for VecPat {
    fn gen_match_body(&self, vec: &Expr, body: &Expr) -> TokenStream {
        match self {
            VecPat::Const(const_)    => const_.gen_match_body(vec, body),
            VecPat::Ident(ident)     => ident.gen_match_body(vec, body),
            VecPat::Lit(lit)         => lit.gen_match_body(vec, body),
            VecPat::Path(path)       => path.gen_match_body(vec, body),
            VecPat::Range(range)     => range.gen_match_body(vec, body),
            VecPat::Reference(ref_)  => ref_.gen_match_body(vec, body),
            VecPat::Rest(est)        => est.gen_match_body(vec, body),
            VecPat::Slice(slice)     => slice.gen_match_body(vec, body),
            VecPat::Struct(struct_)  => struct_.gen_match_body(vec, body),
            VecPat::Tuple(tup)       => tup.gen_match_body(vec, body),
            VecPat::TupleStruct(tup) => tup.gen_match_body(vec, body),
            VecPat::Wild(wild)       => wild.gen_match_body(vec, body),
        }
    }
}

impl GenerateMatchBody for VecPatIdent {
    fn gen_match_body(&self, vec: &Expr, body: &Expr) -> TokenStream {
        let VecPatIdent { ident, .. } = self;
        quote! {
            let #ident = #vec;
            #body
        }
    }
}

impl GenerateMatchBody for VecPatSlice {
    fn gen_match_body(&self, _vec: &Expr, body: &Expr) -> TokenStream {
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
            Some(_) => quote!(let __rem_len = __slice.len();),
            None    => quote!(),
        };

        let before_block = if before.is_empty() {
            quote!()
        } else {
            let mut bindings = TokenStream::default();
            for elem in before {
                bindings.extend(
                    match elem.ident() {
                        Some(ident) => quote! {
                            let #ident = unsafe { __drain.next().unwrap_unchecked() };
                        },
                        None => quote! {
                            let _ = __drain.next();
                        },
                    }
                );
            }

            quote! {
                let mut __drain = __vec.drain(..#before_len);
                #bindings
                #mem_drop(__drain);
            }
        };

        let after_block = if after.is_empty() {
            quote!()
        } else {
            let mut bindings = TokenStream::default();
            for (i, elem) in after.iter().enumerate() {
                bindings.extend(
                    match elem.ident() {
                        Some(ident) => quote! {
                            let #ident = unsafe {
                                __spare[#i].assume_init_read()
                            };
                        },
                        None => quote! {
                            unsafe { __spare[#i].assume_init_drop() };
                        },
                    }
                );
            }

            let new_len_expr = match catchall {
                Some(_) => quote!(__rem_len - #before_len),
                None => quote!(0),
            };

            quote! {
                unsafe { __vec.set_len(#new_len_expr) };
                let __spare = __vec.spare_capacity_mut();
                #bindings
            }
        };

        let bind_block = match catchall {
            Some(Catchall::Ident(ident)) => quote!(let #ident = __vec;),
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
            fn gen_match_body(&self, _vec: &Expr, body: &Expr) -> TokenStream {
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