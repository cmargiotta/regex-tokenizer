pub enum Element {
    ANY,
    RANGE(char, char),
    EXACT(char),
    WHITESPACE,
    DIGIT,
    WORD,
}

pub enum Quantifier {
    ONE(Element),
    STAR(Vec<Element>),
    PLUS(Vec<Element>),
    OPTIONAL(Vec<Element>),
    ROOT,
}

pub struct Node {
    current: Quantifier,
    children: Vec<Node>,
}

struct Token<'a> {
    current: &'a str,
}

impl<'a> Iterator for Token<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.len() == 0 {
            None
        } else {
            let head = &self.current[..1];

            match head {
                r"\" => {
                    let result = &self.current[..2];
                    self.current = &self.current[2..];
                    Some(result)
                }
                _ => {
                    self.current = &self.current[1..];
                    Some(head)
                }
            }
        }
    }
}

// impl Node {
//     pub fn insert(&mut self, regex: &str) -> () {
//         let tokens =
//     }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn regex_tokenization() {
        let tokens: Vec<&str> = super::Token {
            current: r"abc\d[1]",
        }
        .collect();

        assert_eq!(tokens, vec!["a", "b", "c", r"\d", "[", "1", "]"]);
    }
}
