
use crate::Solution;

impl Solution {
    pub fn is_valid_parentheses(s: String) -> bool {
        #[derive(PartialEq, Eq, Clone, Debug)]
        pub struct Bracket {
            kind: BracketKind,
            state: BracketState,
        }

        impl Bracket {
            fn new(kind: BracketKind, state: BracketState) -> Self {
                Self {
                    kind,
                    state,
                }
            }
        }
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        pub enum BracketKind {
            Round,
            Square,
            Curly
        }

        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        pub enum BracketState {
            Open,
            Close
        }

        impl From<char> for Bracket {
            fn from(value: char) -> Self {
                match value {
                    '(' => Bracket::new(BracketKind::Round, BracketState::Open),
                    ')' => Bracket::new(BracketKind::Round, BracketState::Close),
                    '[' => Bracket::new(BracketKind::Square, BracketState::Open),
                    ']' => Bracket::new(BracketKind::Square, BracketState::Close),
                    '{' => Bracket::new(BracketKind::Curly, BracketState::Open),
                    '}' | _ => Bracket::new(BracketKind::Curly, BracketState::Close),
                }
            }
        }
        let mut stack = Vec::new();
        for char in s.chars() {
            let bracket = Bracket::from(char);
            match bracket.state {
                BracketState::Open => stack.push(bracket.kind),
                BracketState::Close => {
                    let Some(old_bracket) = stack.pop() else { return false };
                    if bracket.kind != old_bracket { return false; }
                }
            }
        }
        stack.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        let s = "()[]{}".to_string();
        let answer = Solution::is_valid_parentheses(s);
        let should_be = true;
        assert_eq!(should_be, answer);
    }
    #[test]
    fn test_2() {
        let s = "([]{}".to_string();
        let answer = Solution::is_valid_parentheses(s);
        let should_be = false;
        assert_eq!(should_be, answer);
    }
}