
use crate::Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        pub struct Generator {
            results: Vec<String>,
            str: String,
            n: usize
        }

        impl Generator {
            fn new(n: usize) -> Self {
                Self {
                    n,
                    results: vec![],
                    str: String::with_capacity(n * 2)
                }
            }
            fn generate(&mut self, close: usize, open: usize) {
                if close == open && close == self.n {
                    self.results.push(self.str.clone());
                    return;
                }
                if open < self.n {
                    self.str.push('(');
                    self.generate(close, open + 1);
                    self.str.pop();
                }
                if close < open {
                    self.str.push(')');
                    self.generate(close + 1, open );
                    self.str.pop();
                }
            } 
        }
        let mut generator = Generator::new(n as usize);
        generator.generate(0, 0);
        generator.results

    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

   
    #[test]
    fn test_1() {
        let n = 1;
        let answer = Solution::generate_parenthesis(n);
        let should_be: Vec<String> = ["()"].iter().map(|s| s.to_string()).collect();
        assert_eq!(should_be, answer);
    }
    #[test]
    fn test_2() { 
        let n = 2;
        let answer = Solution::generate_parenthesis(n);
        let should_be: Vec<String> = ["(())", "()()"].iter().map(|s| s.to_string()).collect();
        assert_eq!(should_be, answer);
    }

    #[test]
    fn test_3() {
        let n = 3;
        let answer = Solution::generate_parenthesis(n);
        let should_be: Vec<String> = ["((()))","(()())","(())()","()(())","()()()"].iter().map(|s| s.to_string()).collect();
        assert_eq!(should_be, answer);
    }
    #[test]
    fn test_4() {
        let n = 100;
        let answer = Solution::generate_parenthesis(n);
        let should_be: Vec<String> = ["((()))","(()())","(())()","()(())","()()()"].iter().map(|s| s.to_string()).collect();
        assert_eq!(should_be, answer);
    }
}