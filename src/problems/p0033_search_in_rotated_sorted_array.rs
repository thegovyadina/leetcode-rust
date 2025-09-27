//! # LeetCode Problem: 33 - Search in Rotated Sorted Array
//!
//! Difficulty: Medium
//!
//! Link: https://leetcode.com/problems/search-in-rotated-sorted-array/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(log n) - We use binary search to find the target.
//! - Space Complexity: O(1) - We use a constant amount of space for the pointers `first`,
//!   `last`, and `middle`.
pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // As soon as `log n` is mentioned, we should think of binary search
        let mut first = 0;
        let mut last = nums.len() as i32 - 1;

        while first <= last {
            let middle = (last + first) / 2;
            if nums[middle as usize] == target {
                return middle;
            }
            // Check if the left half is sorted
            if nums[first as usize] <= nums[middle as usize] {
                // Target is in the left half
                if nums[first as usize] <= target && target < nums[middle as usize] {
                    last = middle - 1;
                } else {
                    first = middle + 1;
                }
            } else {
                // Target is in the right half
                if nums[middle as usize] < target && target <= nums[last as usize] {
                    first = middle + 1;
                } else {
                    last = middle - 1;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_search() {
        let test_cases = [
            (vec![4, 5, 6, 7, 0, 1, 2], 0, 4),
            (vec![4, 5, 6, 7, 0, 1, 2], 3, -1),
            (vec![1], 0, -1),
        ];
        for (idx, (input, n, expected)) in test_cases.iter().enumerate() {
            let result = Solution::search(input.clone(), *n);
            assert_eq!(
                result, *expected,
                "Test case #{idx}: with input {input:?}, expected {expected:?}, got {result:?}"
            );
        }
    }
}
