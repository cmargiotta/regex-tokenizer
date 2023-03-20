pub struct Token<TypeEnum> {
    pub value: String,
    pub position: usize,
    pub type_: TypeEnum,
}

pub struct Matcher<TypeEnum, const size: usize> {
    pub matchers: [(regex::Regex, TypeEnum); size],
}

pub trait ValidMatcher {
    type TokenType;

    fn try_match(&self, query: &String, position: usize) -> Option<Self::TokenType>;
}

impl<TypeEnum, const size: usize> ValidMatcher for Matcher<TypeEnum, size> {
    type TokenType = Token<TypeEnum>;

    fn try_match(&self, query: &String, position: usize) -> Option<Self::TokenType> {
        for (regex, type_) in self.matchers {
            let m = regex.find(&query);
            if let Some(token) = m {
                return Some(Token {
                    value: String::from(token.as_str()),
                    position,
                    type_,
                });
            }
        }

        None
    }
}
