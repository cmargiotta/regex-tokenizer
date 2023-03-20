use crate::compilation_error::error;
use proc_macro2::{Span, TokenStream, TokenTree};
use quote::quote;
use regex::Regex;

pub struct Data {
    pub regex: TokenTree,
    pub type_: Option<TokenTree>,
}

pub struct Parsing {
    pub name: TokenTree,
    pub data: Vec<Data>,
}

pub fn parse(code: TokenStream) -> Result<Parsing, TokenStream> {
    let code = proc_macro2::TokenStream::from(code);
    let mut tokens = code.into_iter();

    let name = match tokens.next() {
        Some(token @ TokenTree::Ident(_)) => token,
        None => return Err(error(Span::call_site(), "Tokenizer identifier required")),
        Some(token) => {
            return Err(error(
                token.span(),
                "Unexpected token, an identifier is needed",
            ))
        }
    };

    let mut res: Vec<Data> = Default::default();

    loop {
        let regex: TokenTree = match tokens.next() {
            Some(token @ TokenTree::Literal(_)) => match Regex::new(token.to_string().as_str()) {
                Ok(_) => token,
                Err(err) => {
                    return Err(error(
                        token.span(),
                        format!("Invalid regex: {err}").as_str(),
                    ))
                }
            },
            None => break,
            Some(token) => return Err(error(token.span(), "Unexpected token")),
        };

        assert!(tokens.next().unwrap().to_string() == "=");
        assert!(tokens.next().unwrap().to_string() == ">");

        let type_ = match tokens.next() {
            None => todo!("Wrong syntax"),
            Some(token @ TokenTree::Ident(_)) => match token.to_string().as_str() {
                "_" => None,
                _ => Some(token),
            },
            Some(token) => todo!("Unexpected token {:?}", token),
        };

        res.push(Data { regex, type_ });
    }

    Result::Ok(Parsing { name, data: res })
}

impl Parsing {
    fn get_enum_name(&self) -> proc_macro2::Ident {
        quote::format_ident!("{}_types", self.name.to_string())
    }

    fn get_enum(&self) -> TokenStream {
        let values: Vec<TokenTree> = self
            .data
            .iter()
            .filter(|data| data.type_.is_some())
            .map(|data| data.type_.clone().unwrap().into())
            .collect();

        let name = self.get_enum_name();

        quote! {
            #[derive(Debug)]
            enum #name {
                #(#values),*
            }
        }
    }

    fn get_matchers_initializer(&self) -> proc_macro2::TokenStream {
        let valid = self.data.iter().filter(|data| data.type_.is_some());

        let regexes: Vec<TokenTree> = valid.clone().map(|data| data.regex.clone()).collect();
        let types: Vec<TokenTree> = valid.map(|data| data.type_.clone().unwrap()).collect();
        let enum_type = self.get_enum_name();

        quote! {
            [#((regex_tokenizer::regex::Regex::new(String::from("^") + #regexes).unwrap(), #enum_type::#types),)*]
        }
    }

    pub fn get_parser(&self) -> proc_macro2::TokenStream {
        let enum_ = self.get_enum();
        let name = &self.name;
        let enum_name = self.get_enum_name();
        let regexes_number = self.data.len();
        let matchers = self.get_matchers_initializer();

        quote! {
            #enum_

            type #name = regex_tokenizer::Matcher<#enum_name, #regexes_number>;

            impl<#enum_name, #regexes_number> #name {
                fn new() -> #name {
                    #name {
                        matchers: #matchers
                    }
                }
            }

            impl regex_tokenizer::ValidTokenizer for #name {}
        }
    }
}
