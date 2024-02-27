

use crate::Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        #[derive(PartialEq, Eq, Clone, Debug)]
        pub enum Expression {
            Number (i32),
            Operator (Operator)
        }
        #[derive(PartialEq, Eq, Clone, Debug)]
        #[repr(u8)]
        pub enum Operator {
            Div,
            Sum,
            Mul,
            Sub
        }

        impl From<&str> for Expression {
            fn from(value: &str) -> Self {
                match value {
                    "+" => Expression::Operator(Operator::Sum),
                    "-" => Expression::Operator(Operator::Sub),
                    "/" => Expression::Operator(Operator::Div),
                    "*" => Expression::Operator(Operator::Mul),
                    number => {
                        let number = number.parse().unwrap();
                        Expression::Number(number)
                    }
                }
            }
        }

        let mut stack = Vec::new();
        for t in tokens {
            let expr = Expression::from(t.as_str());
            match expr {
                Expression::Number(n) => stack.push(n),
                Expression::Operator(op) => {
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    match op {
                        Operator::Div => stack.push(b / a),
                        Operator::Sub => stack.push(b - a),
                        Operator::Mul => stack.push(b * a),
                        Operator::Sum => stack.push(b + a),
                    }
                }
            }
        }
        stack.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        let s = vec!["4","13","5","/","+"].into_iter().map(|s| s.to_string()).collect();
        let answer = Solution::eval_rpn(s);
        let should_be = 6;
        assert_eq!(should_be, answer);
    }
    #[test]
    fn test_2() {
        let s = vec!["2","1","+","3","*"].into_iter().map(|s| s.to_string()).collect();
        let answer = Solution::eval_rpn(s);
        let should_be = 9;
        assert_eq!(should_be, answer);
    }
}