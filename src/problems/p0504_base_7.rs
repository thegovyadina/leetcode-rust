//! # LeetCode Problem: 504 - Base 7
//!
//! Difficulty: Easy
//!
//! Link: https://leetcode.com/problems/base-7/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(log n) - We repeatedly divide the number by 7 until it becomes 0.
//! - Space Complexity: O(1) - We use a constant amount of extra space
//!         for the result string and variables.

pub struct Solution;

impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }

        let mut n = num;
        let mut result = String::new();
        let is_negative = n < 0;

        if is_negative {
            n = -n;
        }

        while n > 0 {
            result.push_str(&(n % 7).to_string());
            n /= 7;
        }

        if is_negative {
            result.push('-');
        }

        result.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_convert_to_base7() {
        let test_cases = [(100, "202"), (-7, "-10")];
        for (idx, (input, expected)) in test_cases.iter().enumerate() {
            let result = Solution::convert_to_base7(*input);
            assert_eq!(
                result, *expected,
                "Test case #{}: with input {:?}, expected {:?}, got {:?}",
                idx, input, expected, result
            );
        }
    }
}
