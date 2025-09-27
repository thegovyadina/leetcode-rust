//! # LeetCode Problem: 954 - Array of Doubled Pairs
//!
//! Difficulty: Medium
//!
//! Link: https://leetcode.com/problems/array-of-doubled-pairs/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n log n) - We need to sort the keys of the HashMap.
//! - Space Complexity: O(n) - We use a HashMap to store the counts of each number.
pub struct Solution;

impl Solution {
    /// The idea is to count the occurrences of each number in the array and then
    /// try to pair each number with its double.
    /// We can use a HashMap to store the counts of each number.
    /// We will iterate through the sorted keys of the HashMap,
    /// and for each key, we will check if its double exists in the HashMap.
    /// If it does, we will decrement the counts of both the key and its double.
    /// If we can pair all numbers, we return true; otherwise, we return false.
    /// Example:
    ///     input: [4, -2, 2, -4]
    ///     num_count = {-4: 1, -2: 1, 2: 1, 4: 1}
    ///     sorted_keys = [-4, -2, 2, 4]
    ///     We iterate through the sorted keys:
    ///         - For -4, we check if -4 * 2 = -8 exists in the map.
    ///                 It doesn't, so we skip it.
    ///         - For -2, we check if -2 * 2 = -4 exists in the map.
    ///                 It does, so we decrement the counts of -2 and -4.
    ///         - For 2, we check if 2 * 2 = 4 exists in the map.
    ///                 It does, so we decrement the counts of 2 and 4.
    ///         - For 4, we check if 4 * 2 = 8 exists in the map.
    ///                 It doesn't, so we skip it.
    ///        After iterating through all keys, we check if all counts are zero.
    ///               In this case, they are, so we return true.
    pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
        use std::collections::HashMap;
        let mut num_count: HashMap<i32, i32> = HashMap::new();

        // Count occurrences of each number
        for &num in &arr {
            *num_count.entry(num).or_insert(0) += 1;
        }

        // Get all keys and sort them by absolute value
        // This is important to handle negative numbers correctly
        let mut keys: Vec<i32> = num_count.keys().cloned().collect();
        keys.sort_unstable_by_key(|&x| x.abs());

        for &key in &keys {
            if num_count[&key] == 0 {
                continue; // Skip if this key has already been used up
            }

            // For each value x, we need to find its pair.
            // For positive numbers, the pair is 2x.
            // For negative numbers, the pair is 2x (which is more negative).
            let double = key * 2;

            // Check if we have enough occurrences of the target
            if num_count.get(&double).unwrap_or(&0) < &num_count[&key] {
                return false;
            }

            // Update the counts
            *num_count.entry(double).or_insert(0) -= num_count[&key];
            *num_count.entry(key).or_insert(0) = 0;
        }
        // All counts should be zero
        num_count.values().all(|&count| count == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_can_reorder_doubled() {
        let test_cases = [
            (vec![3, 1, 3, 6], false),
            (vec![2, 1, 2, 6], false),
            (vec![4, -2, 2, -4], true),
        ];
        for (idx, (arr, expected)) in test_cases.iter().enumerate() {
            let result = Solution::can_reorder_doubled(arr.clone());
            assert_eq!(
                result, *expected,
                "Test case #{idx}: with cells {arr:?}, expected {expected:?}, got {result:?}"
            );
        }
    }
}
