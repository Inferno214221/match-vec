extern crate proc_macro;

use match_vec_internal::codegen::make_match;
use proc_macro::TokenStream;
use syn::{ExprMatch, parse_macro_input};

#[proc_macro]
pub fn match_vec(input: TokenStream) -> TokenStream {
    make_match(parse_macro_input!(input as ExprMatch)).into()
}