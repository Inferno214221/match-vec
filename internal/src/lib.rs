use syn::{Arm, Expr, braced, parse::{Parse, ParseStream}, token::Brace};

pub struct MatchArgs {
    expr: Box<Expr>,
    brace_token: Brace,
    arms: Vec<Arm>,
}

impl Parse for MatchArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let expr = Expr::parse_without_eager_brace(input)?;

        let content;
        let brace_token = braced!(content in input);

        let arms = parse_multiple_arms(&content)?;

        Ok(MatchArgs {
            expr: Box::new(expr),
            brace_token,
            arms,
        })
    }
}

pub fn parse_multiple_arms(input: ParseStream) -> syn::Result<Vec<Arm>> {
    let mut arms = Vec::new();
    while !input.is_empty() {
        arms.push(input.call(Arm::parse)?);
    }
    Ok(arms)
}

pub fn make_match(args: MatchArgs) -> proc_macro2::TokenStream {
    todo!()
}