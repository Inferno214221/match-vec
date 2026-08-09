use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Expr;

use crate::{GenerateMatchBody, MatchArgs, VecArm, VecPat, VecPatIdent, VecPatSlice};

impl ToTokens for MatchArgs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let MatchArgs { attrs, expr, brace_token: _, arms } = self;
        let arms = arms.iter().map(|arm| arm.to_tokens_ext(expr));
        tokens.extend(quote! {
            let mut __vec = #expr;
            #(#attrs)*
            match &__vec[..] {
                #(#arms)*
            }
        });
    }
}

impl VecArm {
    pub fn to_tokens_ext(&self, vec: &Expr) -> TokenStream {
        let VecArm { attrs, pat, fat_arrow_token, body, comma } = self;
        let body = pat.gen_match_body(vec, body);
        quote! {
            #(#attrs)*
            #pat #fat_arrow_token {
                // TODO: implement the body here
                #body
            } #comma
        }
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

impl ToTokens for VecPatIdent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let VecPatIdent { attrs, by_ref: _, mutability: _, ident: _, subpat } = self;
        let inner = if let Some((at, boxed)) = subpat {
            quote!(__slice #at #boxed)
        } else {
            quote!(_)
        };
        tokens.extend(quote! {
            #(#attrs)*
            #inner
        });
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