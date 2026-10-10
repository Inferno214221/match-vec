use std::error::Error;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::codegen::VecPatGuard;

use super::{GenerateMatchBody, MatchArgs, VecArm, VecPat, VecPatIdent, VecPatSlice};

pub fn tokenize_error(res: Result<TokenStream, Box<dyn Error>>) -> TokenStream {
    match res {
        Ok(tokens) => tokens,
        Err(err) => {
            let err = format!("{}", err);
            quote! {
                ::std::compile_error!(#err)
            }
        },
    }
}

impl ToTokens for MatchArgs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        #[allow(non_snake_case)]
        let SliceExt = quote!(::match_vec::internal::SliceExt);

        let MatchArgs { attrs, expr, brace_token: _, arms } = self;
        tokens.extend(quote! {
            {
                let mut __match_vec_slice = #SliceExt::assert_trait_and_move(#expr);
                #(#attrs)*
                match &__match_vec_slice[..] {
                    #(#arms)*
                }
            }
        });
    }
}

impl ToTokens for VecArm {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let VecArm { attrs, pat, fat_arrow_token, body, comma } = self;
        let body = tokenize_error(pat.gen_match_body(body));
        tokens.extend(quote! {
            #(#attrs)*
            // Prevents a false positive for borrows that are later shadowed with owned versions.
            // Unfortunately, it also leads to ignoring true positives.
            #[allow(unused_variables)]
            #pat #fat_arrow_token {
                #body
            } #comma
        });
    }
}

impl ToTokens for VecPat {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            VecPat::Const(const_)    => const_.to_tokens(tokens),
            VecPat::Guard(guard)     => guard.to_tokens(tokens),
            VecPat::Ident(ident)     => ident.to_tokens(tokens),
            VecPat::Lit(lit)         => lit.to_tokens(tokens),
            VecPat::Path(path)       => path.to_tokens(tokens),
            VecPat::Range(range)     => range.to_tokens(tokens),
            VecPat::Reference(ref_)  => ref_.to_tokens(tokens),
            VecPat::Rest(rest)       => rest.to_tokens(tokens),
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
        let VecPatIdent { attrs, by_ref: _, mutability: _, ident, subpat } = self;
        let subpat = if let Some((at, boxed)) = subpat {
            quote!(#at #boxed)
        } else {
            quote!()
        };
        tokens.extend(quote! {
            #(#attrs)*
            #ident #subpat
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

impl ToTokens for VecPatGuard {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let VecPatGuard { attrs, pat, if_token, guard } = self;
        tokens.extend(quote! {
            #(#attrs)*
            #pat #if_token #guard
        });
    }
}