//! # LeetCode Problem: 263 - Ugly Number
//!
//! Difficulty: Easy
//!
//! Link: https://leetcode.com/problems/ugly-number/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(log n) - We repeatedly divide the number by 2, 3, and 5
//!   until it becomes 1 or less.
//! - Space Complexity: O(1) - We use a constant amount of space for the result.

pub struct Solution;

impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        let mut n = n;
        while n % 2 == 0 {
            n /= 2;
        }
        while n % 3 == 0 {
            n /= 3;
        }
        while n % 5 == 0 {
            n /= 5;
        }
        n == 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_ugly() {
        let test_cases = [(6, true), (1, true), (14, false)];
        for (idx, (input, expected)) in test_cases.iter().enumerate() {
            let result = Solution::is_ugly(*input);
            assert_eq!(
                result, *expected,
                "Test case #{idx}: with input {input:?}, expected {expected:?}, got {result:?}"
            );
        }
    }
}
