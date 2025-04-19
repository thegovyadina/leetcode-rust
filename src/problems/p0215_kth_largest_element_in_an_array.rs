//! # LeetCode Problem: 215 - Kth Largest Element in an Array
//!
//! Difficulty: Medium
//!
//! Link: https://leetcode.com/problems/kth-largest-element-in-an-array/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n) - The `select_nth_unstable` is claimed to be O(n) for all inputs due to
//!   a fallback mechanism (refer to the Rust documentation for `select_nth_unstable`).
//! - Space Complexity: O(1) - We do not use any additional space that grows with the input size.
//!
//! One of the best explanations of the approaches Sort and Select, Min-Heap, Quickselect is given
//! by `vanAmsen` on LeetCode:
//! https://leetcode.com/problems/kth-largest-element-in-an-array/solutions/3906260/100-3-approaches-video-heap-quickselect-sorting/
pub struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let index = nums.len() - k as usize;
        *nums.select_nth_unstable(index).1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_kth_largest() {
        let test_cases = [
            (vec![3, 2, 1, 5, 6, 4], 2, 5),
            (vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4, 4),
            (vec![11, 22, 33, 11, 11, 44, 56, 52, 61], 8, 11),
        ];
        for (idx, (input, n, expected)) in test_cases.iter().enumerate() {
            let result = Solution::find_kth_largest(input.clone(), *n);
            assert_eq!(
                result, *expected,
                "Test case #{}: with input {:?}, expected {:?}, got {:?}",
                idx, input, expected, result
            );
        }
    }
}
