use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::ExprMatch;

use super::MatchArgs;

pub fn make_match(args: ExprMatch) -> TokenStream {
    let args = MatchArgs::try_from(args)
        .expect("macro input uses match syntax that isn't valid for matching against a Vec");
    args.into_token_stream()
}