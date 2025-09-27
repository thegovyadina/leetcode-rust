//! # LeetCode Problem: 905 - Sort Array By Parity
//!
//! Difficulty: Easy
//!
//! Link: https://leetcode.com/problems/sort-array-by-parity/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n) - We traverse the list containing n elements exactly once.
//! - Space Complexity: O(n) - We use a vector to store the result.

pub struct Solution;

impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let mut odd: Vec<i32> = vec![];
        for num in nums.iter() {
            if num % 2 == 0 {
                result.push(*num);
            } else {
                odd.push(*num);
            }
        }
        result.append(&mut odd);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_sort_array_by_parity() {
        let test_cases = [(vec![3, 1, 2, 4], vec![2, 4, 3, 1]), (vec![0], vec![0])];
        for (idx, (input, expected)) in test_cases.iter().enumerate() {
            let result = Solution::sort_array_by_parity(input.clone());
            assert_eq!(
                result, *expected,
                "Test case #{idx}: with input {input:?}, expected {expected:?}, got {result:?}"
            );
        }
    }
}
