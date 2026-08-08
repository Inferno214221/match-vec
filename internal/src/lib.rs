#![feature(iterator_try_collect)]

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Arm, Attribute, Expr, ExprMatch, Ident, Pat, PatConst, PatIdent, PatLit, PatPath, PatRange, PatReference, PatRest, PatSlice, PatStruct, PatTuple, PatTupleStruct, PatWild, Token, punctuated::Punctuated, token::{Brace, Bracket}};

pub struct MatchArgs {
    pub attrs: Vec<Attribute>,
    pub expr: Box<Expr>,
    pub brace_token: Brace,
    pub arms: Vec<VecArm>,
}

impl TryFrom<ExprMatch> for MatchArgs {
    type Error = ();

    fn try_from(value: ExprMatch) -> Result<Self, Self::Error> {
        let ExprMatch {
            attrs, match_token: _, expr, brace_token, arms
        } = value;
        Ok(MatchArgs {
            attrs, expr, brace_token,
            arms: arms.into_iter().map(VecArm::try_from).try_collect()?
        })
    }
}

impl ToTokens for MatchArgs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let MatchArgs { attrs, expr, brace_token: _, arms } = self;
        let arms = arms.iter().map(|arm| arm.to_tokens_ext(expr));
        tokens.extend(quote! {
            #(#attrs)*
            match &#expr[..] {
                #(#arms)*
            }
        });
    }
}

pub struct VecArm {
    pub attrs: Vec<Attribute>,
    pub pat: VecPat,
    pub fat_arrow_token: Token![=>],
    pub body: Box<Expr>,
    pub comma: Option<Token![,]>,
}

impl VecArm {
    pub fn to_tokens_ext(&self, vec: &Expr) -> TokenStream {
        let VecArm { attrs, pat, fat_arrow_token, body, comma } = self;
        let body = pat.impl_body(vec, body);
        quote! {
            #(#attrs)*
            #pat #fat_arrow_token {
                // TODO: implement the body here
                #body
            } #comma
        }
    }
}

impl TryFrom<Arm> for VecArm {
    type Error = ();

    fn try_from(value: Arm) -> Result<Self, Self::Error> {
        let Arm {
            attrs, pat, fat_arrow_token, body, comma
        } = value;
        Ok(VecArm {
            attrs, fat_arrow_token, body, comma,
            pat: pat.try_into()?
        })
    }
}

// impl ToTokens for VecArm {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         let VecArm { attrs, pat, fat_arrow_token, body, comma } = self;
//         // let body = pat.impl_body(todo!(), body);
//         tokens.extend(quote! {
//             #(#attrs)*
//             #pat #fat_arrow_token {
//                 // TODO: implement the body here
//                 #body
//             } #comma
//         });
//     }
// }

pub enum VecPat {
    Const(PatConst),
    // Guard(PatGuard),
    Ident(VecPatIdent),
    Lit(PatLit),
    // Or(PatOr),
    // Paren(PatParen),
    Path(PatPath),
    Range(PatRange),
    Reference(PatReference),
    Struct(PatStruct),
    Tuple(PatTuple),
    TupleStruct(PatTupleStruct),
    Rest(PatRest),
    Slice(VecPatSlice),
    Wild(PatWild),
}

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

impl TryFrom<Pat> for VecPat {
    type Error = ();

    fn try_from(value: Pat) -> Result<Self, Self::Error> {
        Ok(match value {
            Pat::Const(const_)    => VecPat::Const(const_),
            Pat::Ident(ident)     => VecPat::Ident(VecPatIdent::try_from(ident)?),
            Pat::Lit(lit)         => VecPat::Lit(lit),
            Pat::Path(path)       => VecPat::Path(path),
            Pat::Range(range)     => VecPat::Range(range),
            Pat::Reference(ref_)  => VecPat::Reference(ref_),
            Pat::Rest(rest)       => VecPat::Rest(rest),
            Pat::Slice(slice)     => VecPat::Slice(VecPatSlice::try_from(slice)?),
            Pat::Struct(struct_)  => VecPat::Struct(struct_),
            Pat::Tuple(tup)       => VecPat::Tuple(tup),
            Pat::TupleStruct(tup) => VecPat::TupleStruct(tup),
            Pat::Wild(wild)       => VecPat::Wild(wild),
            _ => Err(())?
        })
    }
}

impl ToTokens for VecPat {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            VecPat::Const(const_)    => const_.to_tokens(tokens),
            VecPat::Ident(ident)     => ident.to_tokens(tokens),
            VecPat::Lit(lit)         => lit.to_tokens(tokens),
            VecPat::Path(path)       => path.to_tokens(tokens),
            VecPat::Range(range)     => range.to_tokens(tokens),
            VecPat::Reference(ref_)  => ref_.to_tokens(tokens),
            VecPat::Rest(est)        => est.to_tokens(tokens),
            VecPat::Slice(slice)     => slice.to_tokens(tokens),
            VecPat::Struct(struct_)  => struct_.to_tokens(tokens),
            VecPat::Tuple(tup)       => tup.to_tokens(tokens),
            VecPat::TupleStruct(tup) => tup.to_tokens(tokens),
            VecPat::Wild(wild)       => wild.to_tokens(tokens),
        }
    }
}

pub struct VecPatIdent {
    pub attrs: Vec<Attribute>,
    pub by_ref: Option<Token![ref]>,
    pub mutability: Option<Token![mut]>,
    pub ident: Ident,
    pub subpat: Option<(Token![@], Box<VecPat>)>,
}

impl TryFrom<PatIdent> for VecPatIdent {
    type Error = ();

    fn try_from(value: PatIdent) -> Result<Self, Self::Error> {
        let PatIdent { attrs, by_ref, mutability, ident, subpat } = value;
        Ok(VecPatIdent {
            attrs, by_ref, mutability, ident,
            subpat: match subpat {
                Some((at, boxed)) => Some((
                    at,
                    Box::new(VecPat::try_from(*boxed)?)
                )),
                None => None,
            }
        })
    }
}

impl ToTokens for VecPatIdent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let VecPatIdent { attrs, by_ref: _, mutability: _, ident: _, subpat } = self;
        let inner = if let Some((at, boxed)) = subpat {
            quote!(slice #at #boxed)
        } else {
            quote!(_)
        };
        tokens.extend(quote! {
            #(#attrs)*
            #inner
        });
    }
}

pub struct VecPatSlice {
    pub attrs: Vec<Attribute>,
    pub bracket_token: Bracket,
    pub elems: Punctuated<VecPat, Token![,]>,
}

impl TryFrom<PatSlice> for VecPatSlice {
    type Error = ();

    fn try_from(value: PatSlice) -> Result<Self, Self::Error> {
        let PatSlice { attrs, bracket_token, elems } = value;
        Ok(VecPatSlice {
            attrs, bracket_token,
            elems: elems.into_iter().map(VecPat::try_from).try_collect()?
        })
    }
}

impl ToTokens for VecPatSlice {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let VecPatSlice { attrs, bracket_token: _, elems } = self;
        let elems = elems.iter();
        tokens.extend(quote! {
            #(#attrs)*
            [#(#elems),*]
        });
    }
}

pub trait ImplBody {
    fn impl_body(&self, vec: &Expr, body: &Expr) -> TokenStream;
}

impl ImplBody for VecPat {
    fn impl_body(&self, vec: &Expr, body: &Expr) -> TokenStream {
        match self {
            VecPat::Const(const_)    => const_.impl_body(vec, body),
            VecPat::Ident(ident)     => ident.impl_body(vec, body),
            VecPat::Lit(lit)         => lit.impl_body(vec, body),
            VecPat::Path(path)       => path.impl_body(vec, body),
            VecPat::Range(range)     => range.impl_body(vec, body),
            VecPat::Reference(ref_)  => ref_.impl_body(vec, body),
            VecPat::Rest(est)        => est.impl_body(vec, body),
            VecPat::Slice(slice)     => slice.impl_body(vec, body),
            VecPat::Struct(struct_)  => struct_.impl_body(vec, body),
            VecPat::Tuple(tup)       => tup.impl_body(vec, body),
            VecPat::TupleStruct(tup) => tup.impl_body(vec, body),
            VecPat::Wild(wild)       => wild.impl_body(vec, body),
        }
    }
}

impl ImplBody for VecPatIdent {
    fn impl_body(&self, vec: &Expr, body: &Expr) -> TokenStream {
        let VecPatIdent { ident, .. } = self;
        quote! {
            let #ident = #vec;
            #body
        }
    }
}

impl ImplBody for VecPatSlice {
    fn impl_body(&self, vec: &Expr, body: &Expr) -> TokenStream {
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
            Some(_) => quote!(let rem_len = slice.len();),
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
                            let #ident = unsafe { drain.next().unwrap_unchecked() };
                        },
                        None => quote! {
                            let _ = drain.next();
                        },
                    }
                );
            }

            quote! {
                let mut drain = #vec.drain(..#before_len);
                #bindings
                #mem_drop(drain);
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
                                spare[#i].assume_init_read()
                            };
                        },
                        None => quote! {
                            unsafe { spare[#i].assume_init_drop() };
                        },
                    }
                );
            }

            let new_len_expr = match catchall {
                Some(_) => quote!(rem_len - #before_len),
                None => quote!(0),
            };

            quote! {
                unsafe { vec.set_len(#new_len_expr) };
                let spare = vec.spare_capacity_mut();
                #bindings
            }
        };

        let bind_block = match catchall {
            Some(Catchall::Ident(ident)) => quote!(let #ident = #vec;),
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

macro_rules! impl_body_default {
    ($T:ty) => {
        impl ImplBody for $T {
            fn impl_body(&self, _vec: &Expr, body: &Expr) -> TokenStream {
                quote!(#body)
            }
        }
    };
    ($T:ty, $($N:ty),+ $(,)?) => {
        impl_body_default!($T);
        impl_body_default!($($N),+);
    }
}

impl_body_default! {
    // If this is actually reached, its at top level. How should it work?
    PatLit,

    PatPath,
    PatRest,
    PatWild,
    PatConst,
    PatRange,
    PatReference,
    PatStruct,
    PatTuple,
    PatTupleStruct,
}

pub fn make_match(args: ExprMatch) -> TokenStream {
    let args = MatchArgs::try_from(args)
        .expect("macro input uses match syntax that isn't valid for matching against a Vec");
    args.into_token_stream()
}