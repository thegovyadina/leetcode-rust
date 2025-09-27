//! # LeetCode Problem: 1431 - Kids With the Greatest Number of Candies
//!
//! Difficulty: Easy
//!
//! Link: https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n) - We traverse the list containing n elements exactly once.
//! - Space Complexity: O(n) - We use a vector to store the result.

pub struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        if let Some(&max_candies) = candies.iter().max() {
            candies
                .iter()
                .map(|&c| c + extra_candies >= max_candies)
                .collect()
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_kids_with_candies() {
        let test_cases = [
            (vec![2, 3, 5, 1, 3], 3, vec![true, true, true, false, true]),
            (
                vec![4, 2, 1, 1, 2],
                1,
                vec![true, false, false, false, false],
            ),
            (vec![12, 1, 12], 10, vec![true, false, true]),
        ];
        for (idx, (candies, extra_candies, expected)) in test_cases.iter().enumerate() {
            let result = Solution::kids_with_candies(candies.clone(), *extra_candies);
            assert_eq!(
                result, *expected,
                "Test case #{idx}: with candies={candies:?} extra_candies={extra_candies:?}, expected {expected:?}, got {result:?}"
            );
        }
    }
}
