//! # LeetCode Problem: 1 - Two Sum
//!
//! Difficulty: Easy
//!
//! Link: https://leetcode.com/problems/two-sum/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n) - We traverse the list containing n elements exactly twice.
//!   Since the hash table look-up time is O(1), the overall time complexity is O(n).
//! - Space Complexity: O(n) - The extra space required depends on the number of items
//!   stored in the hash table.

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            if let Some(j) = map.get(&(target - num)) {
                return vec![*j as i32, i as i32];
            }
            map.insert(num, i);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_two_sum() {
        let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_no_solution() {
        let result = Solution::two_sum(vec![1, 2, 3], 7);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_multiple_answers_possible() {
        let result = Solution::two_sum(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]); // Based on the algorithm, this is the expected order
    }
}
