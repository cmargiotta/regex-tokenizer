#[derive(Debug, PartialEq)]
pub(crate) enum Element {
    ANY,
    RANGE(char, char),
    EXACT(char),
    WHITESPACE,
    DIGIT,
    WORD,
    MIXED(Vec<Element>),
}

impl Element {
    pub fn check(&self, char: char) -> bool {
        match self {
            Element::ANY => true,
            Element::RANGE(c1, c2) => char >= *c1 && char <= *c2,
            Element::EXACT(c) => char == *c,
            Element::WHITESPACE => match char {
                ' ' | '\t' | '\n' | '\r' => true,
                _ => false,
            },
            Element::DIGIT => char >= '0' && char <= '9',
            Element::WORD => match char {
                '0'..='9' => true,
                'a'..='z' => true,
                'A'..='Z' => true,
                _ => false,
            },
            Element::MIXED(matchers) => {
                for matcher in matchers {
                    if matcher.check(char) {
                        return true;
                    }
                }
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Element;

    #[test]
    fn range() {
        let m = Element::RANGE('a', 'z');

        assert!(m.check('b'));
        assert!(!m.check('0'));
    }

    #[test]
    fn any() {
        let m = Element::ANY;

        assert!(m.check('b'));
        assert!(m.check('0'));
    }

    #[test]
    fn exact() {
        let m = Element::EXACT('b');

        assert!(m.check('b'));
        assert!(!m.check('0'));
    }
}
