use crate::Solution;

impl Solution {
    pub fn two_sum_2(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut hashmap = HashMap::with_capacity(numbers.len());
        for (i, n) in numbers.iter().enumerate() {
            let dif = target - n;
            let Some(&j) = hashmap.get(&dif) else {
                hashmap.insert(n, i + 1);
                continue;
            };
            return vec![j as i32, i as i32 + 1];
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        let nums = vec![2,7,11,15];
        let t = 9;
        let answer = Solution::two_sum_2(nums, t);
        let should_be = vec![1, 2];
        assert_eq!(should_be, answer);
    }
}