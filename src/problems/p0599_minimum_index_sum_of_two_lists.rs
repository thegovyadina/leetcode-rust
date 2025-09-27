//! # LeetCode Problem: 599 - Minimum Index Sum of Two Lists
//!
//! Difficulty: Easy
//!
//! Link: https://leetcode.com/problems/minimum-index-sum-of-two-lists/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n + m) - We traverse the lists containing n and m elements exactly twice.
//! - Space Complexity: O(min(n, m)) - The extra space required depends on the number of items
//!   stored in the hash map, which is the size of the smaller list.
pub struct Solution;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut indices = std::collections::HashMap::new();
        let mut min_sum = list1.len() + list2.len();
        let mut result: Vec<String> = vec![];

        for (i, element) in list1.iter().enumerate() {
            indices.insert(element, i);
        }

        for (i, element) in list2.iter().enumerate() {
            if let Some(&j) = indices.get(element) {
                let sum = i + j;
                if sum < min_sum {
                    min_sum = sum;
                    result.clear();
                    result.push(element.clone());
                } else if sum == min_sum {
                    result.push(element.clone());
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_restaurant() {
        let test_cases = [
            (
                vec!["happy", "sad", "good"],
                vec!["sad", "happy", "good"],
                vec!["sad", "happy"],
            ),
            (
                vec!["Shogun", "Tapioca Express", "Burger King", "KFC"],
                vec!["KFC", "Shogun", "Burger King"],
                vec!["Shogun"],
            ),
            (
                vec!["Shogun", "Tapioca Express", "Burger King", "KFC"],
                vec![
                    "Piatti",
                    "The Grill at Torrey Pines",
                    "Hungry Hunter Steakhouse",
                    "Shogun",
                ],
                vec!["Shogun"],
            ),
        ];
        for (idx, (list1, list2, expected)) in test_cases.iter().enumerate() {
            let list1_strings: Vec<String> = list1.iter().map(|&s| s.to_string()).collect();
            let list2_strings: Vec<String> = list2.iter().map(|&s| s.to_string()).collect();
            let expected_strings: Vec<String> = expected.iter().map(|&s| s.to_string()).collect();
            let result = Solution::find_restaurant(list1_strings, list2_strings);
            assert_eq!(
                result, expected_strings,
                "Test case #{idx}: with list1 = {list1:?}, list2 = {list2:?}, expected {expected:?}, got {result:?}"
            );
        }
    }
}
