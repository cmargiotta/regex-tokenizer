use std::fmt::Debug;

pub struct Token<TypeEnum>
where
    TypeEnum: Copy + Debug,
{
    pub value: String,
    pub position: usize,
    pub type_: TypeEnum,
}

pub trait ValidToken: ToString {
    fn position(&self) -> (usize, usize);
}

impl<TypeEnum> ToString for Token<TypeEnum>
where
    TypeEnum: Copy + Debug,
{
    fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl<TypeEnum> ValidToken for Token<TypeEnum>
where
    TypeEnum: Copy + Debug,
{
    fn position(&self) -> (usize, usize) {
        (self.position, self.position + self.value.len())
    }
}

/// A matcher capable of extracting tokens and associating them to a class of tokens
pub struct Matcher<TypeEnum> {
    pub(crate) matchers: Vec<(regex::Regex, TypeEnum)>,
    pub(crate) ignored: Vec<regex::Regex>,
}

impl<TypeEnum: Copy> Matcher<TypeEnum> {
    /// Build a new Matcher for TypeEnum, this function should be invoked only by the `tokenizer!` macro
    pub fn build(regexes: Vec<(String, TypeEnum)>, ignored: Vec<String>) -> Self {
        Self {
            matchers: regexes
                .iter()
                .map(|(regex, type_)| (regex::Regex::new(regex.as_str()).unwrap(), type_.clone()))
                .collect(),
            ignored: ignored
                .iter()
                .map(|regex| regex::Regex::new(regex.as_str()).unwrap())
                .collect(),
        }
    }
}

/// A Matcher is buildable without external data
pub trait BuildableMatcher<TypeEnum> {
    /// Build a new Matcher instance
    fn new() -> Matcher<TypeEnum>;
}

pub trait ValidMatcher {
    type TokenType: ValidToken;

    fn try_match(&self, query: &str, position: usize) -> Option<Self::TokenType>;
}

impl<TypeEnum> ValidMatcher for Matcher<TypeEnum>
where
    TypeEnum: Copy + Debug,
{
    type TokenType = Token<TypeEnum>;

    fn try_match(&self, query: &str, position: usize) -> Option<Self::TokenType> {
        for (regex, type_) in &self.matchers {
            let m = regex.find(&query);
            if let Some(token) = m {
                return Some(Token {
                    value: String::from(token.as_str()),
                    position,
                    type_: type_.clone(),
                });
            }
        }

        for regex in &self.ignored {
            let m = regex.find(&query);
            if let Some(token) = m {
                let token = token.as_str();

                return self.try_match(
                    &String::from(query.strip_prefix(token).unwrap()),
                    position + token.len(),
                );
            }
        }

        None
    }
}
