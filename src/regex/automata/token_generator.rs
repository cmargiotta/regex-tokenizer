use std::ops::Range;

pub struct TokenGenerator<'a> {
    current: &'a str,
    position: usize,
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub value: &'a str,
    pub span: Range<usize>,
}

impl<'a> TokenGenerator<'a> {
    pub fn new(current: &'a str) -> Self {
        Self {
            current,
            position: 0,
        }
    }
}

impl<'a> Iterator for TokenGenerator<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.len() == 0 {
            None
        } else {
            let head = &self.current[..1];

            match head {
                r"\" => {
                    let value = &self.current[..2];
                    let span = self.position..(self.position + 2);

                    self.current = &self.current[2..];
                    self.position += 2;

                    Some(Token { value, span })
                }
                _ => {
                    let span = self.position..(self.position + 1);

                    self.current = &self.current[1..];
                    self.position += 1;

                    Some(Token { value: head, span })
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn tokenization() {
        let tokens: Vec<super::Token> = super::TokenGenerator {
            current: r"abc\d[1]",
            position: 0,
        }
        .collect();

        assert_eq!(
            tokens,
            vec![
                super::Token {
                    value: "a",
                    span: (0..1)
                },
                super::Token {
                    value: "b",
                    span: (1..2)
                },
                super::Token {
                    value: "c",
                    span: (2..3)
                },
                super::Token {
                    value: r"\d",
                    span: (3..5)
                },
                super::Token {
                    value: "[",
                    span: (5..6)
                },
                super::Token {
                    value: "1",
                    span: (6..7)
                },
                super::Token {
                    value: "]",
                    span: (7..8)
                }
            ]
        );
    }
}
