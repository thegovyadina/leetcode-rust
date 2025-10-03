//! # LeetCode Problem: 1910 - Remove All Occurrences of a Substring
//!
//! Difficulty: Medium
//!
//! Link: https://leetcode.com/problems/remove-all-occurrences-of-a-substring/
//!
//! ### Complexity Analysis
//!
//! Stack-based solution:
//!
//! - Time Complexity: O(n * m) - We traverse the string n times and remove part m times.
//! - Space Complexity: O(n) - The extra space required depends on the number of items
//!   stored in the stack.
//!
//! According to the benchmark (`cargo bench --bench p1910`),
//! in some cases, the stack-based solution demonstrates superior performance
//! compared to the direct string manipulation approach.
//! Also, LeetCode claims that the stack-based solution is more memory-efficient.
//! Execute `. ./profile.sh p1910_memory 1 2` to run the memory benchmark locally and probably
//! see some different results.
pub struct Solution;

impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut result = s;

        // Continue removing part from result until it no longer exists
        while let Some(pos) = result.find(&part) {
            // Remove the part at position pos
            result = result[..pos].to_string() + &result[pos + part.len()..];
        }

        result
    }

    pub fn remove_occurrences_stack(s: String, part: String) -> String {
        let mut stack: Vec<char> = Vec::new();
        let mut part_chars: Vec<char> = part.chars().collect();
        let last = part_chars.pop().unwrap(); // Get the last character of the part
        let length = part_chars.len();

        for c in s.chars() {
            if c != last {
                // If current character is not the last character of part, push to stack
                stack.push(c);
            } else {
                // If we have enough characters in the stack and they match part_chars
                if stack.len() >= length {
                    let start_idx = stack.len() - length;
                    let matches = stack[start_idx..]
                        .iter()
                        .zip(part_chars.iter())
                        .all(|(&a, &b)| a == b);

                    if matches {
                        // Pop the matching prefix from the stack
                        for _ in 0..length {
                            stack.pop();
                        }
                    } else {
                        // Not a match, push the current character
                        stack.push(c);
                    }
                } else {
                    // Stack doesn't have enough characters, push current
                    stack.push(c);
                }
            }
        }

        // Convert the stack back to a string
        stack.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_remove_occurrences() {
        let test_cases = [
            ("daabcbaabcbc", "abc", "dab"),
            ("axxxxyyyyb", "xy", "ab"),
            ("gjzgbpggjzgbpgsvpwdk", "gjzgbpg", "svpwdk"),
            (
                "wwwwwwwwwwwwwwwwwwwwwvwwwwswxwwwwsdwxweeohapwwzwuwajrnogb",
                "w",
                "vsxsdxeeohapzuajrnogb",
            ),
        ];
        for (idx, (s, part, expected)) in test_cases.iter().enumerate() {
            let result = Solution::remove_occurrences(s.to_string(), part.to_string());
            assert_eq!(
                result, *expected,
                "Test case #{idx}: with s={s:?}, part={part:?}, expected {expected:?}, got {result:?}"
            );
            let result = Solution::remove_occurrences_stack(s.to_string(), part.to_string());
            assert_eq!(
                result, *expected,
                "Test case #{idx}: with s={s:?}, part={part:?}, expected {expected:?}, got {result:?}"
            );
        }
    }
}
