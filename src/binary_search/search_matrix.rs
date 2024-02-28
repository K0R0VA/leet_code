use crate::Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut low = 0;
        let row_len =  matrix[0].len();
        let row_count =  matrix.len();
        let mut high = row_count * row_len;
        while low <= high {
            let mid = (low + high) / 2;
            let i = std::cmp::min(mid / row_len, row_count - 1);
            let j = std::cmp::min(mid % row_len, row_len - 1);
            let n = matrix[i][j];
            if n == target {
                return true;
            }
            if target > n {
                low = mid + 1;
            } else {
                if high == 0 {
                    break;
                }
                high = mid - 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        let nums = [
            [1,3,5,7],      // i = (6 / matrix_len) - 1 j = 
            //     mid = 6; i = 1; j = 2
            [10,11,16,20],
            //  30 {n = 9; i = 2; j = 1} fn(n) -> (i, j) = (n / 4, n % 4) 
            [23,30,34,60]
        ].iter().map(|a| a.to_vec()).collect();
        let t = 30;
        let answer = Solution::search_matrix(nums, t);
        let should_be = true;
        assert_eq!(should_be, answer);
    }

    #[test]
    fn test_2() {
        let nums = [
                [1,3,5,7],
                [10,11,16,20],
                [23,30,34,60]
            ].iter().map(|a| a.to_vec()).collect();
        let t = 3;
        let answer = Solution::search_matrix(nums, t);
        let should_be = true;
        assert_eq!(should_be, answer);
    }
    #[test]
    fn test_3() {
        let nums = [[1], [1]].iter().map(|a| a.to_vec()).collect();
        let t = 0;
        let answer = Solution::search_matrix(nums, t);
        let should_be = false;
        assert_eq!(should_be, answer);
    }

    

    #[test]
    fn test_4() {
        let nums = [[1]].iter().map(|a| a.to_vec()).collect();
        let t = 2;
        let answer = Solution::search_matrix(nums, t);
        let should_be = false;
        assert_eq!(should_be, answer);
    }

    #[test]
    fn test_5() {
        let nums = [[1, 1]].iter().map(|a| a.to_vec()).collect();
        let t = 0;
        let answer = Solution::search_matrix(nums, t);
        let should_be = false;
        assert_eq!(should_be, answer);
    }

    #[test]
    fn test_6() {
        let nums = [[1, 3]].iter().map(|a| a.to_vec()).collect();
        let t = 1;
        let answer = Solution::search_matrix(nums, t);
        let should_be = true;
        assert_eq!(should_be, answer);
    }
    #[test]
    fn test_7() {
        let nums = [[1], [3], [5]].iter().map(|a| a.to_vec()).collect();
        let t = 5;
        let answer = Solution::search_matrix(nums, t);
        let should_be = true;
        assert_eq!(should_be, answer);
    }
}