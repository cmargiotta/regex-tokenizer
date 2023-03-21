pub mod matcher;

use std::marker::PhantomData;

use matcher::ValidToken;

pub struct TokenGenerator<'a, 'b, T: matcher::ValidMatcher, TypeEnum>
where
    'a: 'b,
{
    query: &'b str,
    matcher: &'a T,
    current_position: usize,
    _dummy: PhantomData<TypeEnum>,
}

pub trait Tokenizer<'a, 'b, T: matcher::ValidMatcher> {
    /// Prepare an iterator to extract tokens from a string
    ///
    /// ```
    /// # use regex_tokenizer::tokenizer;
    /// tokenizer! {
    ///     Test
    ///
    ///     r"[a-zA-Z]\w*" => Identifier
    ///     r"\d+" => Number
    ///     r"\s+" => _
    /// }
    ///
    /// let tokenizer = Test::new();
    /// let query = "Identifier  11";
    ///
    /// let mut tokens = tokenizer.tokenize(query);
    /// let token = tokens.next().unwrap();
    /// ```
    fn tokenize(&'a self, data: &'b str) -> TokenGenerator<'a, 'b, T, T::TokenType>;
}

impl<'a, 'b, T> Tokenizer<'a, 'b, T> for T
where
    T: matcher::ValidMatcher,
    'a: 'b,
{
    fn tokenize(&'a self, data: &'b str) -> TokenGenerator<'a, 'b, T, T::TokenType> {
        TokenGenerator {
            query: data,
            matcher: &self,
            current_position: 0,
            _dummy: Default::default(),
        }
    }
}

impl<'a, 'b, T: matcher::ValidMatcher, TypeEnum> Iterator for TokenGenerator<'a, 'b, T, TypeEnum> {
    type Item = T::TokenType;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.matcher.try_match(&self.query, self.current_position);

        match res {
            Some(result) => {
                self.current_position = result.position().1;
                self.query = &self.query[result.to_string().len()..];
                Some(result)
            }
            None => None,
        }
    }
}
