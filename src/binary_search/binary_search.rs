use crate::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let index = -1;
        let mut low = 0;
        let mut high = nums.len() ;
        while low <= high {
            let mid = (low + high) / 2;
            let Some(&n) = nums.get(mid) else {break;};
            if n == target {
                return mid as i32;
            }
            if target > n {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        index
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        let nums = vec![-1,0,3,5,9,12];
        let t = 9;
        let answer = Solution::search(nums, t);
        let should_be = 4;
        assert_eq!(should_be, answer);
    }
}