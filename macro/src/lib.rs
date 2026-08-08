extern crate proc_macro;

use match_vec_internal::{MatchArgs, make_match};
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn match_vec(input: TokenStream) -> TokenStream {
    make_match(parse_macro_input!(input as MatchArgs)).into()
}