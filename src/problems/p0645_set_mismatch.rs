//! # LeetCode Problem: 645 - Set Mismatch
//!
//! Difficulty: Easy
//!
//! Link: https://leetcode.com/problems/set-mismatch/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n) - We traverse the list containing n elements exactly twice.
//! - Space Complexity: O(1) - We use a constant amount of extra space.

pub struct Solution;

#[allow(clippy::needless_range_loop)]
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
