//! # LeetCode Problem: 645 - Set Mismatch
//!
//! You have a set of integers s, which originally contains all the numbers from 1 to n.
//! Unfortunately, due to some error, one of the numbers in s got duplicated
//! to another number in the set, which results in repetition of one number
//! and loss of another number.
//!
//! You are given an integer array nums representing the data status of this set after the error.
//!
//! Find the number that occurs twice and the number that is missing and return them
//! in the form of an array.
//!
//! Example 1:
//!     Input: nums = [1,2,2,4]
//!     Output: [2,3]
//!
//! Example 2:
//!     Input: nums = [1,1]
//!     Output: [1,2]
//!
//! Constraints:
//!     2 <= nums.length <= 104
//!     1 <= nums[i] <= 104
//!
//! Difficulty: Easy
//!
//! Link: https://leetcode.com/problems/set-mismatch/

pub struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut result = vec![];
        for i in 0..nums.len() {
            let idx = nums[i].unsigned_abs() as usize - 1;
            if nums[idx] < 0 {
                result.push(idx as i32 + 1);
            } else {
                nums[idx] *= -1;
            }
        }
        for i in 0..nums.len() {
            if nums[i] > 0 {
                result.push(i as i32 + 1);
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_error_nums() {
        let test_cases = [
            (vec![1, 2, 2, 4], vec![2, 3]), // Duplicate: 2, Missing: 3
            (vec![1, 1], vec![1, 2]),       // Duplicate: 1, Missing: 2
            (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10], vec![10, 11]),
        ];
        for (idx, (input, expected)) in test_cases.iter().enumerate() {
            let result = Solution::find_error_nums(input.clone());
            assert_eq!(
                result, *expected,
                "Test case #{}: with input {:?}, expected {:?}, got {:?}",
                idx, input, expected, result
            );
        }
    }
}
