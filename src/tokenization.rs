pub mod matcher;

use std::marker::PhantomData;

use proc_macro2::{Span, TokenStream};

use self::matcher::Token;

pub struct TokenGenerator<'a, T: matcher::ValidMatcher, TypeEnum> {
    query: TokenStream,
    matcher: &'a T,
    current_position: usize,
    _dummy: PhantomData<TypeEnum>,
}

pub trait Tokenizer<T: matcher::ValidMatcher, TypeEnum> {
    fn tokenize(&self, data: TokenStream) -> TokenGenerator<T, TypeEnum>;
}

impl<T: matcher::ValidMatcher, TypeEnum> Tokenizer<T, TypeEnum> for T {
    fn tokenize(&self, data: TokenStream) -> TokenGenerator<T, TypeEnum> {
        TokenGenerator {
            query: data,
            matcher: &self,
            current_position: 0,
            _dummy: Default::default(),
        }
    }
}

impl<'a, T: matcher::ValidMatcher, TypeEnum> Iterator for TokenGenerator<'a, T, TypeEnum> {
    type Item = Token<TypeEnum>;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self
            .matcher
            .try_match(&self.query.to_string(), self.current_position);

        match res {
            Some(result) => {
                self.current_position += res.value.len();
                res
            }
        }
    }
}
