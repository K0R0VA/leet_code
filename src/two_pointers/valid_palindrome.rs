


use crate::Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars = s.chars()
            .filter(|char| char.is_alphabetic() || char.is_numeric())
            .map(|char| char.to_ascii_lowercase());
        let rev_chars = s.chars()
            .filter(|char| char.is_alphabetic() || char.is_numeric())
            .map(|char| char.to_ascii_lowercase())
            .rev();
        let mut stream = chars.zip(rev_chars); 
        while let Some((char, rev)) = stream.next() {
            if char != rev { return false; }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        let str = "A man, a plan, a canal: Panama".to_string();
        let answer = Solution::is_palindrome(str);
        let should_be = true;
        assert_eq!(should_be, answer);
    }
}