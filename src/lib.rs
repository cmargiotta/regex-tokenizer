//! A regex-based tokenizer.
//!
//! ```
//! use regex_tokenizer::tokenizer;
//!
//! tokenizer! {
//!     Test
//!
//!     r"[a-zA-Z]\w*" => Identifier
//!     r"\d+" => Number
//!     r"\s+" => _
//! }
//!
//! let tokenizer = Test::new();
//! let query = "Identifier  11";
//!
//! let mut tokens = tokenizer.tokenize(query);
//! let token = tokens.next().unwrap();
//!
//! assert_eq!(token.position, 0);
//! assert_eq!(token.value, "Identifier");
//! assert_eq!(token.type_, Test_types::Identifier);
//!
//! let token = tokens.next().unwrap();
//!
//! assert_eq!(token.position, 12);
//! assert_eq!(token.value, "11");
//! assert_eq!(token.type_, Test_types::Number);
//! ```

mod regex;
mod tokenization;

pub use regex_tokenizer_impl::*;

pub use tokenization::matcher::BuildableMatcher;
pub use tokenization::matcher::Matcher;
pub use tokenization::Tokenizer;
