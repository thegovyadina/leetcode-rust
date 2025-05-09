//! # LeetCode Problem: 2471 - Count pairs of Points with Distance k
//!
//! Difficulty: Medium
//!
//! Link: https://leetcode.com/problems/count-pairs-of-points-with-distance-k/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n*k) - We iterate through the list of coordinates, and for each coordinate,
//!         we check all possible values for x_xor from 0 to k.
//! - Space Complexity: O(n) - We use a HashMap to store the frequency of each coordinate.

pub struct Solution;

impl Solution {
    pub fn count_pairs(coordinates: Vec<Vec<i32>>, k: i32) -> i32 {
        use std::collections::HashMap;

        let mut count = 0;
        let mut point_map: HashMap<(i32, i32), i32> = HashMap::new();

        for point in &coordinates {
            let x1 = point[0];
            let y1 = point[1];

            // Try all possible values for x_xor from 0 to k
            for x_xor in 0..=k {
                let y_xor = k - x_xor;

                // Calculate the coordinates that would give these XOR values
                let x2 = x1 ^ x_xor;
                let y2 = y1 ^ y_xor;

                // Check if we've seen a point that would form a valid pair with the current point
                if let Some(&freq) = point_map.get(&(x2, y2)) {
                    count += freq;
                }
            }

            // Add the current point to the map
            *point_map.entry((x1, y1)).or_insert(0) += 1;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_count_pairs() {
        let test_cases = [
            (vec![vec![1, 2], vec![4, 2], vec![1, 3], vec![5, 2]], 5, 2),
            (
                vec![vec![1, 3], vec![1, 3], vec![1, 3], vec![1, 3], vec![1, 3]],
                0,
                10,
            ),
        ];
        for (idx, (coordinates, k, expected)) in test_cases.iter().enumerate() {
            let result = Solution::count_pairs(coordinates.clone(), *k);
            assert_eq!(
                result, *expected,
                "Test case #{}: with coordinates {:?}, k={:?}, expected {:?}, got {:?}",
                idx, coordinates, k, expected, result
            );
        }
    }
}
