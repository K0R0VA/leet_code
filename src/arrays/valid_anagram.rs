
use crate::Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        use std::collections::HashMap;
        if s.len() != t.len() { return false; }
        let mut set = HashMap::with_capacity(s.len());
        for symbol in s.chars() { 
            set.entry(symbol).and_modify(|len| { *len += 1 }).or_insert(1);
        }
        for symbol in t.chars() {
            let Some(lengths) = set.get_mut(&symbol) else {return false};
            *lengths -= 1;
            if *lengths == 0 {
                set.remove(&symbol);
            }
        }
        set.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();
        let answer = Solution::is_anagram(s, t);
        let should_be = true;
        assert_eq!(should_be, answer);
    }
}