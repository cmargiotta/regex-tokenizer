extern crate proc_macro;

mod compilation_error;
mod parse_declaration;

use proc_macro::TokenStream;
use quote::quote;

/// Parse a tokenizer DSL, where three kinds of statements are admitted:
/// - `IDENTIFIER`: a valid Rust identified used to give a name to the Tokenizer and related internal types
/// - `"regex" => Type`: "regex" is a valid regular expression, Type is a valid Rust identifier. Tokens that match the regex will be considered of type `Type`
/// - `"regex" => _`: "regex" is a valid regular expression and tokens that match it are considered separators and ignored.
///
/// # Examples
/// ```
/// tokenizer! {
///     Test
///
///     r"[a-zA-Z]\w*" => Identifier
///     r"\d+" => Number
///     r"\s+" => _
/// }
/// ```
#[proc_macro]
pub fn tokenizer(_item: TokenStream) -> TokenStream {
    let data = match parse_declaration::parse(_item.into()) {
        Ok(data) => data,
        Err(error) => return error.into(),
    };

    let parser = data.get_parser();

    quote! {
        #parser
    }
    .into()
}
