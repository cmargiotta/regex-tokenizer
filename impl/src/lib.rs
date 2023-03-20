extern crate proc_macro;

mod compilation_error;
mod parse_declaration;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn tokenizer(_item: TokenStream) -> TokenStream {
    let data = match parse_declaration::parse(_item.into()) {
        Ok(data) => data,
        Err(error) => return error.into(),
    };

    let parser = data.get_parser();

    println!("{}", parser.to_string());

    quote! {
        #parser
    }
    .into()
}
