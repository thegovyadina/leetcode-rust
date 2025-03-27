//! # LeetCode Problem: 1137 - Nth Tribonacci Number
//!
//! Difficulty: Easy
//!
//! Link: https://leetcode.com/problems/nth-tribonacci-number/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n) - We iterate through the numbers from 3 to n.
//! - Space Complexity: O(1) - We use a constant amount of space
//!         for the variables `a`, `b`, and `c`.

pub struct Solution;

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        match n {
            0 => 0,
            1 | 2 => 1,
            _ => {
                let mut a = 0;
                let mut b = 1;
                let mut c = 1;
                for _ in 3..=n {
                    (a, b, c) = (b, c, a + b + c);
                }
                c
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_tribonacci() {
        let test_cases = [(25, 1389537), (4, 4), (18, 19513)];
        for (idx, (input, expected)) in test_cases.iter().enumerate() {
            let result = Solution::tribonacci(*input);
            assert_eq!(
                result, *expected,
                "Test case #{}: with input {:?}, expected {:?}, got {:?}",
                idx, input, expected, result
            );
        }
    }
}
