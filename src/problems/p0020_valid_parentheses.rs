//! # LeetCode Problem: 20 - Valid Parentheses
//!
//! Difficulty: Easy
//!
//! Link: https://leetcode.com/problems/valid-parentheses/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n) - We traverse the string containing n characters exactly once.
//! - Space Complexity: O(n) - In the worst case, we need to store all opening
//!   parentheses in the stack.

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for symbol in s.chars() {
            match symbol {
                '{' => stack.push('}'),
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '}' | ')' | ']' => {
                    if Some(symbol) != stack.pop() {
                        return false;
                    }
                }
                _ => (),
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_valid() {
        let test_cases = [
            ("()", true),
            ("[]]", false),
            ("()[]{}", true),
            ("(]", false),
            ("([)]", false),
            ("{[]}", true),
            ("{[()]}", true),
            ("{[}", false),
            ("", true),
        ];
        for (idx, (input, expected)) in test_cases.iter().enumerate() {
            let result = Solution::is_valid(input.to_string());
            assert_eq!(
                result, *expected,
                "Test case #{idx}: with input {input:?}, expected {expected:?}, got {result:?}"
            );
        }
    }
}
