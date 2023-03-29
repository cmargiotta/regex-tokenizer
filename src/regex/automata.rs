mod element;
mod quantifier;
mod token_generator;

use element::Element;
use quantifier::Quantifier;
use std::{collections::VecDeque, ops::Range};
use token_generator::{Token, TokenGenerator};

#[derive(Debug, PartialEq, Clone)]
pub enum NodeKind<Type: Clone + PartialEq> {
    FINAL(Type),
    SKIP,
}

#[derive(Debug, PartialEq)]
pub struct Node<TypeEnum: Clone + PartialEq> {
    current: Quantifier,
    kind: NodeKind<TypeEnum>,
    children: Vec<Node<TypeEnum>>,
}

#[derive(Debug, PartialEq)]
pub struct Error<'a> {
    token: Token<'a>,
    cause: &'static str,
}

impl<TypeEnum: Clone + PartialEq> Node<TypeEnum> {
    fn new_skip(current: Quantifier) -> Self {
        Self {
            current,
            kind: NodeKind::SKIP,
            children: Default::default(),
        }
    }

    fn new_root() -> Self {
        Self {
            current: Quantifier::empty(),
            kind: NodeKind::SKIP,
            children: Default::default(),
        }
    }

    fn new_automata<'a>(string: &'a str, id: TypeEnum) -> Result<Self, Error<'a>> {
        let mut tokens = TokenGenerator::new(string);

        let mut result = Self::new_root();

        let mut nodes: Vec<Node<TypeEnum>> = Default::default();

        while let Some(token) = tokens.next() {
            match token {
                t @ Token { value: "?", .. } => {
                    //Optional node
                    if let Some(last) = nodes.last_mut() {
                        last.current.min_repetitions = 0;
                        last.current.max_repetitions = Some(1);
                    } else {
                        return Err(Error {
                            token: t,
                            cause: "Wrong syntax",
                        });
                    }
                }
                t @ Token { value: "+", .. } => {
                    // + node
                    if let Some(last) = nodes.last_mut() {
                        last.current.min_repetitions = 1;
                        last.current.max_repetitions = None;
                    } else {
                        return Err(Error {
                            token: t,
                            cause: "Wrong syntax",
                        });
                    }
                }
                t @ Token { value: "*", .. } => {
                    // * node
                    if let Some(last) = nodes.last_mut() {
                        last.current.min_repetitions = 0;
                        last.current.max_repetitions = None;
                    } else {
                        return Err(Error {
                            token: t,
                            cause: "Wrong syntax",
                        });
                    }
                }
                Token { value: "[", .. } => {
                    // [...] block
                    let mut content: VecDeque<Token> = Default::default();

                    let _ok = tokens.find_map(|token| {
                        if token.value == "]" {
                            //Closing ] found, stop iterating with find_map
                            return Some(true);
                        }

                        content.push_back(token);
                        None
                    });

                    let mut elements: Vec<Element> = Default::default();
                    while let Some(token) = content.pop_front() {
                        if let Some(token2) = content.front().clone() {
                            let char1 = token.value.chars().next().unwrap();
                            match token2.value {
                                "-" => {
                                    let token2 = content.pop_front().unwrap();

                                    if let Some(token3) = content.pop_front() {
                                        //token - token3 range identified
                                        let char2 = token3.value.chars().next().unwrap();

                                        if char2 < char1 {
                                            return Err(Error {
                                                token: token2,
                                                cause: "Invalid range [a-b], b must be >= a",
                                            });
                                        } else {
                                            elements.push(Element::RANGE(char1, char2));
                                        }
                                    } else {
                                        return Err(Error {
                                            token: token2,
                                            cause: "Invalid range, wrong format",
                                        });
                                    }
                                }
                                "]" => {
                                    content.pop_front();
                                    elements.push(Element::EXACT(char1));
                                }
                                _ => elements.push(Element::EXACT(char1)),
                            }
                        }
                    }

                    nodes.push(Node::new_skip(Quantifier::one(Element::MIXED(elements))));
                }
                Token { value: r"\d", .. } => {
                    // Digit node
                    nodes.push(Node::new_skip(Quantifier::one(Element::DIGIT)))
                }
                Token { value: r"\s", .. } => {
                    // Space node
                    nodes.push(Node::new_skip(Quantifier::one(Element::WHITESPACE)))
                }
                Token { value: r"\w", .. } => {
                    // Word node
                    nodes.push(Node::new_skip(Quantifier::one(Element::WORD)))
                }
                Token { value, .. } => nodes.push(Node::new_skip(Quantifier::one(Element::EXACT(
                    // Exact node
                    value.chars().next().unwrap(),
                )))),
            }
        }

        if !nodes.is_empty() {
            nodes.last_mut().unwrap().kind = NodeKind::FINAL(id);

            let mut node = nodes.pop().unwrap();

            while !nodes.is_empty() {
                let mut n = nodes.pop().unwrap();
                n.children.push(node);
                node = n;
            }

            result.children.push(node);
        }

        Ok(result)
    }

    fn add_child(&mut self, mut child: Self) -> &mut Self {
        let kind = child.kind.clone();
        let node_index = self
            .children
            .iter()
            .position(|c| {
                if c.current == child.current {
                    if let NodeKind::FINAL(_) = c.kind {
                        child.kind == NodeKind::SKIP
                    } else {
                        true
                    }
                } else {
                    false
                }
            })
            .unwrap_or_else(|| {
                self.children.push(child);
                self.children.len() - 1
            });

        let node = &mut self.children[node_index];

        if node.kind == NodeKind::SKIP {
            node.kind = kind;
        }

        node
    }

    pub fn add_matcher<'a>(&mut self, string: &'a str, id: TypeEnum) -> Result<(), Error<'a>> {
        let mut new_children = Self::new_automata(string, id)?.children.into_iter();

        let mut current = self;

        while let Some(child) = new_children.next() {
            current = current.add_child(child);
        }

        Ok(())
    }

    fn try_match_impl<'a>(&self, string: &'a str, position: usize) -> Option<(&'a str, TypeEnum)> {
        let (ok, pos) = self.current.try_match(&string[position..]);

        if !ok {
            None
        } else {
            if let NodeKind::FINAL(id) = &self.kind {
                Some((&string[0..pos], id.clone()))
            } else {
                for child in &self.children {
                    if let res @ Some(_) = child.try_match_impl(string, pos) {
                        return res;
                    }
                }

                None
            }
        }
    }

    pub fn try_match<'a>(&self, string: &'a str) -> Option<(&'a str, TypeEnum)> {
        self.try_match_impl(string, 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::regex::automata::{element::Element, quantifier::Quantifier, Node, NodeKind};

    #[test]
    fn automata_building() {
        #[derive(Debug, PartialEq, Clone)]
        enum A {
            T,
        }

        let automata = super::Node::<A>::new_automata(r"a[Xa-z]", A::T).unwrap();
        assert_eq!(
            automata,
            super::Node::<A> {
                current: Quantifier {
                    elements: vec![],
                    min_repetitions: 0,
                    max_repetitions: Some(0)
                },
                kind: NodeKind::SKIP,
                children: vec![Node {
                    current: Quantifier {
                        elements: vec![Element::EXACT('a')],
                        min_repetitions: 1,
                        max_repetitions: Some(1)
                    },
                    kind: NodeKind::SKIP,
                    children: vec![Node {
                        current: Quantifier {
                            elements: vec![Element::MIXED(vec![
                                Element::EXACT('X'),
                                Element::RANGE('a', 'z')
                            ])],
                            min_repetitions: 1,
                            max_repetitions: Some(1)
                        },
                        kind: NodeKind::FINAL(A::T),
                        children: vec![]
                    }]
                },]
            }
        )
    }

    #[test]
    fn automata_matching() {
        #[derive(Debug, PartialEq, Clone)]
        enum A {
            T,
        }

        let automata = super::Node::<A>::new_automata(r"a\d[a-z]", A::T).unwrap();

        assert_eq!(automata.try_match("a1zaaaa"), Some(("a1z", A::T)));
    }

    #[test]
    fn automata_expansion() {
        #[derive(Debug, PartialEq, Clone)]
        enum A {
            T,
            U,
        }

        let mut automata = super::Node::<A>::new_automata(r"a[Xa-z]", A::T).unwrap();
        automata.add_matcher("a", A::U).expect("");

        assert_eq!(
            automata,
            super::Node::<A> {
                current: Quantifier {
                    elements: vec![],
                    min_repetitions: 0,
                    max_repetitions: Some(0)
                },
                kind: NodeKind::SKIP,
                children: vec![Node {
                    current: Quantifier {
                        elements: vec![Element::EXACT('a')],
                        min_repetitions: 1,
                        max_repetitions: Some(1)
                    },
                    kind: NodeKind::FINAL(A::U),
                    children: vec![Node {
                        current: Quantifier {
                            elements: vec![Element::MIXED(vec![
                                Element::EXACT('X'),
                                Element::RANGE('a', 'z')
                            ])],
                            min_repetitions: 1,
                            max_repetitions: Some(1)
                        },
                        kind: NodeKind::FINAL(A::T),
                        children: vec![]
                    }]
                },]
            }
        )
    }
}
