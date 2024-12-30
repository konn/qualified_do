use proc_macro::TokenStream;

mod desugar;
mod parser;
mod types;

use types::*;

use syn::parse_macro_input;

#[proc_macro]
pub fn qdo(input: TokenStream) -> TokenStream {
    let qdo: QDo = parse_macro_input!(input as QDo);
    qdo.desugar().map_or_else(
        |err: syn::Error| TokenStream::from(err.into_compile_error()),
        |a| a.into(),
    )
}
