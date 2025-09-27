//! # LeetCode Problem: 1876 - Substrings of Size Three with Distinct Characters
//!
//! Difficulty: Easy
//!
//! Link: https://leetcode.com/problems/substrings-of-size-three-with-distinct-characters/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n) - We iterate through the string once, checking each substring of size 3.
//! - Space Complexity: O(1) - We use a constant amount of space for the substring check.

pub struct Solution;

impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        if s.len() < 3 {
            return 0;
        }
        let bytes = s.as_bytes();
        let mut count = 0;
        for i in 0..bytes.len().saturating_sub(2) {
            if bytes[i] != bytes[i + 1] && bytes[i] != bytes[i + 2] && bytes[i + 1] != bytes[i + 2]
            {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_count_good_substrings() {
        let test_cases = [("xyzzaz", 1), ("aababcabc", 4), ("wr", 0)];
        for (idx, (input, expected)) in test_cases.iter().enumerate() {
            let result = Solution::count_good_substrings(input.to_string());
            assert_eq!(
                result, *expected,
                "Test case #{idx}: with input {input:?}, expected {expected:?}, got {result:?}"
            );
        }
    }
}
