


use crate::Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) ->  Vec<i32> {
        let n = temperatures.len();
        let mut stack: Vec<usize> = vec![];
        let mut res = vec![0; n];
        for i in 0 .. n {
            while let Some(&top) = stack.last() {
                if temperatures[i] > temperatures[top] {
                    stack.pop();
                    res[top] = (i - top) as i32;
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        let temperatures = vec![73,74,74,75,71,69,72,76,73];
        let answer = Solution::daily_temperatures(temperatures);
        let should_be = vec![1,2,1,4,2,1,1,0,0];
        assert_eq!(should_be, answer);
    }
    #[test]
    fn test_2() {
        let temperatures = vec![77,41,41,41,41,77,41,77,77,77];
        let answer = Solution::daily_temperatures(temperatures);
        let should_be = vec![0,4,3,2,1,0,1,0,0,0];
        assert_eq!(should_be, answer);
    }
}