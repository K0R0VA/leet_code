
use crate::Solution;

impl Solution {
    // FIRST SOLUTION (Time Limit Exceeded) must write an algorithm that runs in O(n)
    // pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    //     let mut result = vec![1; nums.len()];
    //     for (i, n) in nums.into_iter().enumerate() {
    //         for (_, r) in result.iter_mut().enumerate().filter(|(j, _)| *j != i) {
    //             *r *= n;
    //         }
    //     }
    //     result
    // }
    // Second SOLUTION 
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let size = len - 1;
        let mut result = vec![1; len];
        let mut left_product = vec![1; len];
        let mut right_product = vec![1; len];
        for i in 0 .. len {
            if i + 1 <= len - 1 {
                left_product[i + 1] *= nums[i] * left_product[i]
            }
            let j = size - i;
            if j > 0 {
                right_product[j - 1] *= nums[j] * right_product[j]
            }
        }
        for i in 0 ..= len - 1 {
            result[i] = left_product[i] * right_product[i];
        }
        result
    }
    // BETTER SOLUTION
    // pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    //     let mut result = vec![1; nums.len()];
    //     for i in 1..nums.len() {
    //         result[i] = result[i - 1] * nums[i - 1];
    //     }
    //     let mut right = 1;
    //     for i in (0..nums.len()).rev() {
    //         result[i] *= right;
    //         right *= nums[i];
    //     }
    //     result
    // }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        let request = vec![2, 1, 1, 2, 3, 4];
        let answer = Solution::product_except_self(request);
        let should_be = vec![24, 48, 48, 24, 16, 12];
        assert_eq!(should_be, answer);
    }
}