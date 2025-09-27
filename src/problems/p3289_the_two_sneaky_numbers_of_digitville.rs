//! # LeetCode Problem: 3289 - The Two Sneaky Numbers of Digitville
//!
//! Difficulty: Easy
//!
//! Link: https://leetcode.com/problems/the-two-sneaky-numbers-of-digitville/
//!
//! ## Complexity Analysis
//!
//! The main solution:
//! - Time Complexity: O(n) - We traverse the list containing n elements exactly once.
//! - Space Complexity: O(1) - We use a constant amount of extra space.
//!
//! The HashSet solution:
//! - Time Complexity: O(n) - We traverse the list containing n elements exactly once.
//! - Space Complexity: O(n) - We use a HashSet to store the numbers.

pub struct Solution;
pub struct HashSetSolution;

impl Solution {
    /// This is a memory-efficient solution that uses a boolean vector
    /// to track the numbers.
    /// The idea is to iterate through the list and mark the numbers as seen.
    /// If we encounter a number that has already been seen,
    /// we add it to the result.
    /// We stop when we have found two numbers.
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        // We use a boolean vector to track the numbers.
        // According to the problem constraints, the maximum length is 102.
        let mut appearances: Vec<bool> = vec![false; 102];
        for &n in nums.iter() {
            if appearances[n as usize] {
                result.push(n);
            } else {
                appearances[n as usize] = true;
            }

            if result.len() == 2 {
                return result;
            }
        }
        result
    }
}

impl HashSetSolution {
    /// This is my first solution.
    /// It's time-efficient, as it uses a HashSet to track the numbers,
    /// but it requires additional memory.
    /// Left for comparison.
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashSet;
        let mut appearances: HashSet<i32> = HashSet::new();
        let mut result: Vec<i32> = vec![];
        for &n in nums.iter() {
            if appearances.contains(&n) {
                result.push(n);
            } else {
                appearances.insert(n);
            }

            if result.len() == 2 {
                return result;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::{HashSetSolution, Solution};

    #[test]
    fn test_get_sneaky_numbers() {
        let test_cases = [
            (vec![0, 1, 1, 0], vec![0, 1]),
            (vec![0, 3, 2, 1, 3, 2], vec![2, 3]),
            (vec![7, 1, 5, 4, 3, 4, 6, 0, 9, 5, 8, 2], vec![4, 5]),
        ];
        for (idx, (input, expected)) in test_cases.iter().enumerate() {
            let mut result = Solution::get_sneaky_numbers(input.clone());
            let mut expected_result = expected.clone();
            let mut result2 = HashSetSolution::get_sneaky_numbers(input.clone());
            let mut expected_result2 = expected.clone();
            result.sort();
            expected_result.sort();
            result2.sort();
            expected_result2.sort();
            assert_eq!(
                result, expected_result,
                "Test case for the main Solution #{idx}: with input {input:?}, expected {expected:?}, got {result:?}"
            );
            assert_eq!(
                result2, expected_result2,
                "Test case for the HashSetSolution #{idx}: with input {input:?}, expected {expected:?}, got {result2:?}"
            );
        }
    }
}
