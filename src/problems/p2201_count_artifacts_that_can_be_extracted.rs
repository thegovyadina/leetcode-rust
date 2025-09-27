//! # LeetCode Problem: 2201 - Count Artifacts That Can Be Extracted
//!
//! Difficulty: Medium
//!
//! Link: https://leetcode.com/problems/count-artifacts-that-can-be-extracted/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(m * n) - We iterate through each artifact and check
//!   if all its cells have been dug up.
//! - Space Complexity: O(m * n) - We use a HashSet to store dug positions,
pub struct Solution;

impl Solution {
    pub fn dig_artifacts(n: i32, artifacts: Vec<Vec<i32>>, dig: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;
        if n == 0 {
            return 0;
        }
        // Create a set of dug positions for a quick lookup
        let dig_positions: HashSet<(i32, i32)> =
            dig.into_iter().map(|pos| (pos[0], pos[1])).collect();

        // Count fully extracted artifacts
        artifacts
            .iter()
            .filter(|&artifact| {
                // Check if all cells of this artifact have been dug up
                let (r1, c1, r2, c2) = (artifact[0], artifact[1], artifact[2], artifact[3]);

                for r in r1..=r2 {
                    for c in c1..=c2 {
                        if !dig_positions.contains(&(r, c)) {
                            return false; // Found a cell that hasn't been dug
                        }
                    }
                }
                true // All cells have been dug
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_dig_artifacts() {
        let test_cases = [
            (
                2,
                vec![vec![0, 0, 0, 0], vec![0, 1, 1, 1]],
                vec![vec![0, 0], vec![0, 1]],
                1,
            ),
            (
                2,
                vec![vec![0, 0, 0, 0], vec![0, 1, 1, 1]],
                vec![vec![0, 0], vec![0, 1], vec![1, 1]],
                2,
            ),
        ];
        for (idx, (n, artifacts, dig, expected)) in test_cases.iter().enumerate() {
            let result = Solution::dig_artifacts(*n, artifacts.clone(), dig.clone());
            assert_eq!(
                result, *expected,
                "Test case #{idx}: with n {n:?}, artifacts {artifacts:?}, dig {dig:?}, expected {expected:?}, got {result:?}"
            );
        }
    }
}
