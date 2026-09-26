use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::ExprMatch;

use crate::codegen::to_tokens::tokenize_error;

use super::MatchArgs;

pub fn make_match(args: ExprMatch) -> TokenStream {
    tokenize_error(MatchArgs::try_from(args).map(MatchArgs::into_token_stream))
}