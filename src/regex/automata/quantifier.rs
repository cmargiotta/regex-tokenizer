use super::element::Element;

#[derive(Debug, PartialEq)]
pub struct Quantifier {
    pub(crate) elements: Vec<Element>,
    pub min_repetitions: usize,
    pub max_repetitions: Option<usize>,
}

impl Quantifier {
    pub(crate) fn empty() -> Self {
        Quantifier {
            elements: Default::default(),
            min_repetitions: 0,
            max_repetitions: Some(0),
        }
    }

    pub(crate) fn one(element: Element) -> Self {
        Quantifier {
            elements: vec![element],
            min_repetitions: 1,
            max_repetitions: Some(1),
        }
    }

    pub(crate) fn try_match(&self, string: &str) -> (bool, usize) {
        let mut position = 0;
        let mut repetitions = 0;

        let max = if let Some(max) = self.max_repetitions {
            max
        } else {
            usize::MAX
        };

        if self.min_repetitions == 0 && max == 0 {
            return (true, 0);
        }

        while position < string.len() && position < max {
            let last_position = position;

            if !self.elements.iter().all(|element| {
                position += 1;
                element.check(string.chars().nth(position - 1).unwrap())
            }) {
                return (
                    repetitions >= self.min_repetitions && repetitions <= max,
                    last_position,
                );
            }

            repetitions += 1;
        }

        (true, position)
    }
}

#[cfg(test)]
mod tests {
    use crate::regex::automata::element::Element;

    use super::Quantifier;

    #[test]
    fn test() {
        let q = Quantifier {
            elements: vec![Element::RANGE('a', 'z'), Element::RANGE('0', '9')],
            min_repetitions: 2,
            max_repetitions: None,
        };

        assert_eq!(q.try_match("a1z9cc"), (true, 4));
        assert_eq!(q.try_match("a1cc"), (false, 2));
        assert_eq!(q.try_match("cc"), (false, 0));
    }
}
