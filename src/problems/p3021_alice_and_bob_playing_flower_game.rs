//! # LeetCode Problem: 3021 - Alice and Bob Playing Flower Game
//!
//! Difficulty: Medium
//!
//! Link: https://leetcode.com/problems/alice-and-bob-playing-flower-game/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(1) - The solution runs in constant time since it only involves basic
//!   arithmetic operations
//! - Space Complexity: O(1) - The solution uses a constant amount of space for variables.

pub struct Solution;

impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        // Count even numbers from 1 to n and 1 to m
        let even_n = n / 2;
        let even_m = m / 2;

        // Count odd numbers from 1 to n and 1 to m
        let odd_n = (n + 1) / 2;
        let odd_m = (m + 1) / 2;

        // Alice wins when (x is even and y is odd) OR (x is odd and y is even)
        // Number of valid pairs = (even_n * odd_m) + (odd_n * even_m)
        (even_n as i64 * odd_m as i64) + (odd_n as i64 * even_m as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_flower_game() {
        let test_cases = [(3, 2, 3), (1, 1, 0)];
        for (idx, (n, m, expected)) in test_cases.iter().enumerate() {
            let result = Solution::flower_game(*n, *m);
            assert_eq!(
                result, *expected,
                "Test case #{idx}: with n={n:?}, m={m:?}, expected {expected:?}, got {result:?}"
            );
        }
    }
}
