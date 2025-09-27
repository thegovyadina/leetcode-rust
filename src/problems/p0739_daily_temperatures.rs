//! # LeetCode Problem: 739 - Daily Temperatures
//!
//! Difficulty: Medium
//!
//! Link: https://leetcode.com/problems/daily-temperatures/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n) - We traverse the list containing n elements exactly once.
//! - Space Complexity: O(n) - We use a stack to store the indices of the temperatures.

pub struct Solution;

impl Solution {
    /// We use monotonic stack to keep track of the indices of the temperatures.
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; temperatures.len()];
        let mut stack = vec![];

        for i in 0..temperatures.len() {
            while !stack.is_empty() && temperatures[i] > temperatures[*stack.last().unwrap()] {
                let idx = stack.pop().unwrap();
                result[idx] = (i - idx) as i32;
            }
            stack.push(i);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_daily_temperatures() {
        let test_cases = [
            (
                vec![73, 74, 75, 71, 69, 72, 76, 73],
                vec![1, 1, 4, 2, 1, 1, 0, 0],
            ),
            (vec![30, 40, 50, 60], vec![1, 1, 1, 0]),
            (vec![30, 60, 90], vec![1, 1, 0]),
        ];
        for (idx, (input, expected)) in test_cases.iter().enumerate() {
            let result = Solution::daily_temperatures(input.clone());
            assert_eq!(
                result, *expected,
                "Test case #{idx}: with input {input:?}, expected {expected:?}, got {result:?}"
            );
        }
    }
}
