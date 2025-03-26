//! # LeetCode Problem: 3174 - Clear Digits
//!
//! Difficulty: Easy
//!
//! Link: https://leetcode.com/problems/clear-digits/

pub struct Solution;

impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut result: Vec<char> = vec![];
        for char in s.chars() {
            if char.is_ascii_digit() {
                result.pop();
            } else {
                result.push(char);
            }
        }
        result.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_clear_digits() {
        let test_cases = [("abc", "abc"), ("cb34", ""), ("vn45n432vffn42e", "vfe")];
        for (idx, (input, expected)) in test_cases.iter().enumerate() {
            let result = Solution::clear_digits(input.to_string());
            assert_eq!(
                result, *expected,
                "Test case #{}: with input {:?}, expected {:?}, got {:?}",
                idx, input, expected, result
            );
        }
    }
}
