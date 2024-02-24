
use crate::Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut map = HashMap::with_capacity(strs.len());
        for word in strs {
            let mut chars = word.as_bytes().to_vec();
            chars.sort();
            let Some(words) = map.get_mut(&chars) else {
                map.insert(chars, vec![word]);
                continue;
            };
            words.push(word);
        }
        map.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        let strs: Vec<String> = ["eat","tea","tan","ate","nat","bat"].iter().map(|str| str.to_string()).collect();
        let answer = Solution::group_anagrams(strs);
        let should_be = vec![
            vec!["nat","tan"],
            vec!["ate","eat","tea"],
            vec!["bat"],
        ];
        assert_eq!(should_be, answer);
    }
}