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
                #by_ref #mutability #ident
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
        #[allow(non_snake_case)]
        let VecExt = quote!(::match_vec::internal::VecExt);

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

        let pop_bindings = match (&before[..], &after[..]) {
            ([], [])        => quote!(),
            ([], after)     => {
                let after_len = after.len();
                let after_destruct = after.iter().map(|p| p.ident_pat().unwrap_or(quote!(_)));
                quote! {
                    // SAFETY: __match_vec_vec contains #after_len elements due to pattern matching.
                    let [
                        #(#after_destruct),*
                    ] = unsafe {
                        #VecExt::pop_back_n::<#after_len>(&mut __match_vec_vec)
                    };
                }
            },
            (before, [])    => {
                let before_len = before.len();
                let before_destruct = before.iter().map(|p| p.ident_pat().unwrap_or(quote!(_)));
                quote! {
                    // SAFETY: __match_vec_vec contains #before_len elements due to pattern
                    // matching.
                    let [
                        #(#before_destruct),*
                    ] = unsafe {
                        #VecExt::pop_front_n::<#before_len>(&mut __match_vec_vec)
                    };
                }
            },
            (before, after) => {
                let before_len = before.len();
                let after_len = after.len();
                let before_destruct = before.iter().map(|p| p.ident_pat().unwrap_or(quote!(_)));
                let after_destruct = after.iter().map(|p| p.ident_pat().unwrap_or(quote!(_)));
                quote! {
                    // SAFETY: __match_vec_vec contains #before_len + #after_len elements due to
                    // pattern matching.
                    let (
                        [#(#before_destruct),*], [#(#after_destruct),*]
                    ) = unsafe {
                        #VecExt::pop_both::<#before_len, #after_len>(&mut __match_vec_vec)
                    };
                }
            },
        };

        let catchall_binding = if let Some(Catchall::Ident(ident)) = catchall {
            ident.gen_binding()
        } else {
            quote!()
        };

        quote! {
            #pop_bindings
            #catchall_binding
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