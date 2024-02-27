


use crate::Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {return 0;}
        use std::collections::{BinaryHeap, HashSet};
        let mut set = HashSet::with_capacity(nums.len());
        let mut heap = BinaryHeap::with_capacity(nums.len());
        let mut max = 1;
        for n in nums {
            if set.contains(&n) {continue;}
            heap.push(n);
            set.insert(n);
        }
        let mut count = 1;
        let mut previus_n = None;
        let heap_len = heap.len();
        let mut i = heap.len();
        while let Some(n) = heap.pop() {
            i -= 1;
            if Some(n + 1) == previus_n {
                count +=1;
                max = std::cmp::max(max, count);
            } else {
                if count > heap_len - i { return count as i32; }
                count = 1;

            }
            previus_n = Some(n);
        }
        max as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        let request = vec![100,4,200,1,3,2];
        let answer = Solution::longest_consecutive(request);
        let should_be = 4;
        assert_eq!(should_be, answer);
    }
}