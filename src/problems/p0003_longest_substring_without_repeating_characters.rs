//! # LeetCode Problem: 3 - Longest Substring Without Repeating Characters
//!
//! Difficulty: Medium
//!
//! Link: https://leetcode.com/problems/longest-substring-without-repeating-characters/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n) - We traverse the string containing n characters exactly once.
//! - Space Complexity: O(n) - The extra space required depends on the number of items
//!   stored in the hash table.

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() <= 1 {
            return s.len() as i32;
        }

        let mut result = 0;
        let mut left = 0;
        let mut letters = std::collections::HashMap::new();

        for (right, c) in s.chars().enumerate() {
            if let Some(&prev_idx) = letters.get(&c) {
                left = std::cmp::max(prev_idx + 1, left);
            }
            letters.insert(c, right);
            result = std::cmp::max(result, (right - left + 1) as i32);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_length_of_longest_substring() {
        let test_cases = [("abcabcbb", 3), ("bbbbb", 1), ("pwwkew", 3)];
        for (idx, (input, expected)) in test_cases.iter().enumerate() {
            let result = Solution::length_of_longest_substring(input.to_string());
            assert_eq!(
                result, *expected,
                "Test case #{idx}: with input {input:?}, expected {expected:?}, got {result:?}"
            );
        }
    }
}
