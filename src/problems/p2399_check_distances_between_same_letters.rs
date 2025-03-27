//! # LeetCode Problem: 2399 - Check Distances Between Same Letters
//!
//! Difficulty: Easy
//!
//! Link: https://leetcode.com/problems/check-distances-between-same-letters/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n) - We traverse the list containing n elements exactly once.
//! - Space Complexity: O(1) - We use a constant amount of extra space.

pub struct Solution;

impl Solution {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let s_bytes = s.as_bytes();
        // Use an array to track first occurrences of each letter
        let mut first_indices = [-1; 26];

        for (i, &byte) in s_bytes.iter().enumerate() {
            let char_idx = (byte - b'a') as usize;

            if first_indices[char_idx] == -1 {
                // First occurrence of this letter
                first_indices[char_idx] = i as i32;
            } else {
                // Second occurrence - check distance
                let expected_distance = distance[char_idx];
                let actual_distance = (i as i32) - first_indices[char_idx] - 1;

                if actual_distance != expected_distance {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_check_distance() {
        let test_cases = [
            (
                "abaccb",
                vec![
                    1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                true,
            ),
            (
                "aa",
                vec![
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                false,
            ),
            (
                "baacbddc",
                vec![
                    0, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                true,
            ),
        ];
        for (idx, (input_string, input_distance, expected)) in test_cases.iter().enumerate() {
            let result =
                Solution::check_distances(input_string.to_string(), input_distance.to_vec());
            assert_eq!(
                result, *expected,
                "Test case #{}: with input string {:?} and distance {:?}, expected {:?}, got {:?}",
                idx, input_string, input_distance, expected, result
            );
        }
    }
}
