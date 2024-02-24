

use crate::Solution;

impl Solution {
    // FIRST SOLUTION
    // pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    //     #[derive(PartialEq, Eq, Hash, Debug)]
    //     pub struct InverseOrdInt {
    //         count: i32,
    //         n: i32
    //     }
    //     impl PartialOrd for InverseOrdInt {
    //         fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    //             Some(self.cmp(other))
    //         }
    //     }
    //     impl Ord for InverseOrdInt {
    //         fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    //             self.count.cmp(&other.count)
    //         }
    //     }
    //     use std::collections::{HashMap, BinaryHeap, HashSet};
    //     if nums.len() == k as usize { return nums; }
    //     let mut map = HashMap::with_capacity(nums.len());
    //     let mut heap = BinaryHeap::with_capacity(nums.len());
    //     for  &n in nums.iter() {
    //         let Some(count) = map.get_mut(&n) else {
    //             map.insert(n, 1);
    //             heap.push(InverseOrdInt {count: 1, n});
    //             continue;
    //         };
    //         *count += 1;
    //         heap.push(InverseOrdInt {count: *count, n});
    //     }
    //     let mut response = Vec::with_capacity(k as usize);
    //     let mut i = 0;
    //     let mut set = HashSet::with_capacity(nums.len());
    //     while i < k {
    //         let Some(num) = heap.pop() else { continue };
    //         let InverseOrdInt {n , ..} = num;
    //         if !set.contains(&n) {
    //             response.push(n);
    //             set.insert(n);
    //             i += 1;
    //         }
    //     }
    //     response
    // }
    // BETTER SOLUTION
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::<i32, usize>::with_capacity(nums.len());
        let mut bucket = vec![Vec::new(); nums.len() + 1];
        for n in nums {
            *map.entry(n).or_default() += 1;
        }
        map.into_iter().for_each(|(k, v)| bucket[v].push(k));
        bucket.into_iter().rev().flatten().take(k as usize).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        let nums = vec![1,1,1,2,2,3];
        let k = 2;
        let answer = Solution::top_k_frequent(nums, k);
        let should_be = vec![1, 2];
        assert_eq!(should_be, answer);
    }

    #[test]
    fn test_2() {
        let nums = vec![1,2];
        let k = 2;
        let answer = Solution::top_k_frequent(nums, k);
        let should_be = vec![1, 2];
        assert_eq!(should_be, answer);
    }
    #[test]
    fn test_3() {
        let nums = vec![4,1,-1,2,-1,2,3];
        let k = 2;
        let answer = Solution::top_k_frequent(nums, k);
        let should_be = vec![2,-1];
        assert_eq!(should_be, answer);
    }
    #[test]
    fn test_4() {
        let nums = vec![5,3,1,1,1,3,73,1];
        let k = 2;
        let answer = Solution::top_k_frequent(nums, k);
        let should_be = vec![1,3];
        assert_eq!(should_be, answer);
    }
    #[test]
    fn test_5() {
        let nums = vec![3,2,3,1,2,4,5,5,6,7,7,8,2,3,1,1,1,10,11,5,6,2,4,7,8,5,6];
        let k = 10;
        let answer = Solution::top_k_frequent(nums, k);
        let should_be = vec![1,2,5,3,6,7,4,8,10,11];
        assert_eq!(should_be, answer);
    }
}