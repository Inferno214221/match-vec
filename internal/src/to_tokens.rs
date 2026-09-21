use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::{GenerateMatchBody, MatchArgs, VecArm, VecPat, VecPatIdent, VecPatSlice};

// TODO: quote_spanned

impl ToTokens for MatchArgs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        #[allow(non_snake_case)]
        let Vec = quote!(::std::vec::Vec);

        let MatchArgs { attrs, expr, brace_token: _, arms } = self;
        tokens.extend(quote! {
            let mut __match_vec_vec: #Vec<_> = #expr;
            #(#attrs)*
            match &__match_vec_vec[..] {
                #(#arms)*
            }
        });
    }
}

impl ToTokens for VecArm {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let VecArm { attrs, pat, fat_arrow_token, body, comma } = self;
        let body = pat.gen_match_body(body);
        tokens.extend(quote! {
            #(#attrs)*
            #pat #fat_arrow_token {
                // TODO: implement the body here
                #body
            } #comma
        });
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
            quote!(__match_vec_slice #at #boxed)
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