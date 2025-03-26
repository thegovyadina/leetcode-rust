//! # LeetCode Problem: 1974 - Minimum Time to Type Word Using Special Typewriter
//!
//! Difficulty: Easy
//!
//! Link: https://leetcode.com/problems/minimum-time-to-type-word-using-special-typewriter/

pub struct Solution;

impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        let mut result = 0;
        let mut previous = 'a';
        for ch in word.chars() {
            let diff = (ch as i32 - previous as i32).abs();
            result += diff.min(26 - diff) + 1;
            previous = ch;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn min_time_to_type() {
        let test_cases = [("abc", 5), ("bza", 7), ("zjpc", 34)];
        for (idx, (input, expected)) in test_cases.iter().enumerate() {
            let result = Solution::min_time_to_type(input.to_string());
            assert_eq!(
                result, *expected,
                "Test case #{}: with input {:?}, expected {:?}, got {:?}",
                idx, input, expected, result
            );
        }
    }
}
