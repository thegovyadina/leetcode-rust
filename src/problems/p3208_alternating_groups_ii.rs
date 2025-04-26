//! # LeetCode Problem: 3208 - Alternating Groups II
//!
//! Difficulty: Medium
//!
//! Link: https://leetcode.com/problems/alternating-groups-ii/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n + k) - We traverse the list containing n elements and
//!             extend it by k elements.
//! - Space Complexity: O(n + k) - We use a vector to store the extended list.

pub struct Solution;

impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        // We use a sliding window technique to find the number of alternating groups
        let n = colors.len();
        let k = k as usize;

        // Create an extended array to handle circular sequences
        let mut extended_colors = colors.clone();
        // Extend with elements from the start of colors, up to min(k-1, n)
        extended_colors.extend(colors.iter().take(std::cmp::min(k - 1, n)).cloned());

        let length = extended_colors.len();
        let mut result = 0;

        // Initialize the bounds of the sliding window
        let mut left = 0;
        let mut right = 1;

        while right < length {
            // Check if the current color is the same as the last one
            if extended_colors[right] == extended_colors[right - 1] {
                // Pattern breaks, reset a window from the current position
                left = right;
                right += 1;
                continue;
            }

            // Extend the window
            right += 1;

            // Skip counting a sequence if its length is less than k
            if right - left < k {
                continue;
            }

            // Record a valid sequence and shrink the window from the left to search for more
            result += 1;
            left += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_number_of_alternating_groups() {
        let test_cases = [
            (vec![0, 1, 0, 1, 0], 3, 3),
            (vec![0, 1, 0, 0, 1, 0, 1], 6, 2),
            (vec![1, 1, 0, 1], 4, 0),
        ];
        for (idx, (colors, k, expected)) in test_cases.iter().enumerate() {
            let result = Solution::number_of_alternating_groups(colors.clone(), *k);
            assert_eq!(
                result, *expected,
                "Test case #{}: with colors {:?}, k={:?}, expected {:?}, got {:?}",
                idx, colors, k, expected, result
            );
        }
    }
}
