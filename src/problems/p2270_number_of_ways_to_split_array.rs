//! # LeetCode Problem: 2270 - Number of Ways to Split Array
//!
//! Difficulty: Medium
//!
//! Link: https://leetcode.com/problems/number-of-ways-to-split-array/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n) - We iterate through the array once to calculate the prefix sums.
//! - Space Complexity: O(1) - We use a constant amount of space for the prefix sums and counters.

pub struct Solution;

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        // Casting to i64 is required to avoid overflow leading to incorrect results
        // in LeetCode's test cases
        let sum: i64 = nums.iter().map(|&x| x as i64).sum();
        let mut left_sum: i64 = 0;
        let mut count = 0;
        for i in 0..nums.len() - 1 {
            left_sum += nums[i] as i64;
            if left_sum >= sum - left_sum {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn ways_to_split_array() {
        let test_cases = [
            (vec![10, 4, -8, 7], 2),
            (vec![2, 3, 1, 0], 2),
            (vec![5, 13, 10, 6, 12, 4, 4, 2], 5),
            (vec![5, 3, 10, 6, -12, 0, 8, 2], 5),
        ];
        for (idx, (input, expected)) in test_cases.iter().enumerate() {
            let result = Solution::ways_to_split_array(input.clone());
            assert_eq!(
                result, *expected,
                "Test case #{}: with input {:?}, expected {:?}, got {:?}",
                idx, input, expected, result
            );
        }
    }
}
