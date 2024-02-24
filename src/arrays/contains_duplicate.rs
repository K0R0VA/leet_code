
use crate::Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        if nums.len() <=1 { return false; }
        let mut set = HashSet::with_capacity(nums.len());
        for num in nums {
            if set.contains(&num) {return true;}
            set.insert(num);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        let request = vec![1,2,3,1];
        let answer = Solution::contains_duplicate(request);
        let should_be = true;
        assert_eq!(should_be, answer);
    }
}