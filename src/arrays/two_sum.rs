use crate::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, num) in nums.iter().enumerate() {
            for (j, rev_num) in nums.iter().enumerate().rev() {
                let sum = rev_num + num;
                if sum == target && i != j {
                    return vec![i as i32, j as i32]
                }
            }
        }
        vec![]
    }
    // BETTER SOlULTION
    // pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //     let mut map = HashMap::new();

    //     for (index, &num) in nums.iter().enumerate() {
    //         if let Some(&i) = map.get(&(target - num)) {
    //             return vec![i as i32, index as i32];
    //         }
    //         map.insert(num, index);
    //     }

    //     vec![] //Returning empty vector incase no solution is found
    // }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        let nums = vec![2,7,11,15];
        let t = 9;
        let answer = Solution::two_sum(nums, t);
        let should_be = vec![0, 1];
        assert_eq!(should_be, answer);
    }
}