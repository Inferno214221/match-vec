#![feature(iterator_try_collect)]

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Arm, Attribute, Expr, ExprMatch, Ident, Pat, PatIdent, PatLit, PatPath, PatRest, PatSlice, PatWild, Token, punctuated::Punctuated, token::{Brace, Bracket}};

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
        tokens.extend(quote! {
            #(#attrs)*
            match #expr {
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

impl ToTokens for VecArm {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let VecArm { attrs, pat, fat_arrow_token, body, comma } = self;
        // let body = pat.impl_body(todo!(), body);
        tokens.extend(quote! {
            #(#attrs)*
            #pat #fat_arrow_token {
                // TODO: implement the body here
                #body
            } #comma
        });
    }
}

pub enum VecPat {
    // Const(PatConst),
    // Guard(PatGuard),
    Ident(VecPatIdent),
    Lit(PatLit),
    // Or(PatOr),
    // Paren(PatParen),
    Path(PatPath),
    Rest(PatRest),
    Slice(VecPatSlice),
    Wild(PatWild),
}

impl TryFrom<Pat> for VecPat {
    type Error = ();

    fn try_from(value: Pat) -> Result<Self, Self::Error> {
        Ok(match value {
            Pat::Ident(ident) => VecPat::Ident(VecPatIdent::try_from(ident)?),
            Pat::Path(path) => VecPat::Path(path),
            Pat::Rest(rest) => VecPat::Rest(rest),
            Pat::Slice(slice) => VecPat::Slice(VecPatSlice::try_from(slice)?),
            Pat::Wild(wild) => VecPat::Wild(wild),
            _ => Err(())?
        })
    }
}

impl ToTokens for VecPat {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            VecPat::Ident(ident) => ident.to_tokens(tokens),
            VecPat::Lit(lit) => lit.to_tokens(tokens),
            VecPat::Path(path) => path.to_tokens(tokens),
            VecPat::Rest(est) => est.to_tokens(tokens),
            VecPat::Slice(slice) => slice.to_tokens(tokens),
            VecPat::Wild(wild) => wild.to_tokens(tokens),
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
        todo!()
    }
}

pub fn make_match(args: ExprMatch) -> TokenStream {
    let args = MatchArgs::try_from(args)
        .expect("macro input uses match syntax that isn't valid for matching against a Vec");
    args.into_token_stream()
}