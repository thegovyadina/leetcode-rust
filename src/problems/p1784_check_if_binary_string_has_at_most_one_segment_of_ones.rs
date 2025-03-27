//! # LeetCode Problem: 1784 - Check if Binary String Has at Most One Segment of Ones
//!
//! Difficulty: Easy
//!
//! Link: https://leetcode.com/problems/check-if-binary-string-has-at-most-one-segment-of-ones/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n) - We traverse the string containing n characters exactly once.
//! - Space Complexity: O(1) - We use a constant amount of extra space.

pub struct Solution;

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        // The official description is the most difficult part of the problem.
        // The point is to check if there is "01" substring in the string.
        // The shortest implementation is the following:
        //
        //     pub fn check_ones_segment(s: String) -> bool {
        //         !s.contains("01")
        //      }
        //
        // That's it!
        //
        // However, here I pretend that I don't know this function.
        if s.len() == 1 && s == "0" {
            return false;
        }
        let mut previous = '1';
        for char in s.chars() {
            if char == '1' && previous == '0' {
                return false;
            }
            previous = char;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_check_ones_segment() {
        let test_cases = [
            ("1001", false),
            ("1", true),
            ("1100", true),
            ("0011", false),
            ("00000100001", false),
            ("10011101", false),
            ("0", false),
        ];
        for (idx, (input, expected)) in test_cases.iter().enumerate() {
            let result = Solution::check_ones_segment(input.to_string());
            assert_eq!(
                result, *expected,
                "Test case #{}: with input {:?}, expected {:?}, got {:?}",
                idx, input, expected, result
            );
        }
    }
}
