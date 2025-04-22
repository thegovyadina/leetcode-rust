//! # LeetCode Problem: 13 - Roman to Integer
//!
//! Difficulty: Easy
//!
//! Link: https://leetcode.com/problems/roman-to-integer/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n) - We traverse the string containing n characters exactly once.
//! - Space Complexity: O(1) - We use a constant amount of space for the result
//!         and current maximum value.

pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut curr_max = 0;

        for letter in s.chars().rev() {
            let num = match letter {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => panic!("Invalid Roman numeral"),
            };
            if num >= curr_max {
                result += num;
                curr_max = num
            } else {
                result -= num;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_roman_to_int() {
        let test_cases = [("III", 3), ("LVIII", 58), ("MCMXCIV", 1994)];
        for (idx, (input, expected)) in test_cases.iter().enumerate() {
            let result = Solution::roman_to_int(input.to_string());
            assert_eq!(
                result, *expected,
                "Test case #{}: with input {:?}, expected {:?}, got {:?}",
                idx, input, expected, result
            );
        }
    }
}
