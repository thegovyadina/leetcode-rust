//! # LeetCode Problem: 1464 - Maximum Product of Two Elements in an Array
//!
//! Difficulty: Easy
//!
//! Link: https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n) - We traverse the list containing n elements exactly once.
//! - Space Complexity: O(1) - We use a constant amount of extra space.

pub struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut max1, mut max2) = (0, 0);
        for num in nums.iter() {
            if *num > max1 {
                max2 = max1;
                max1 = *num;
            } else if *num > max2 {
                max2 = *num;
            }
        }
        (max1 - 1) * (max2 - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_product() {
        let test_cases = [
            (vec![3, 4, 5, 2], 12),
            (vec![1, 5, 4, 5], 16),
            (vec![3, 7], 12),
        ];
        for (idx, (input, expected)) in test_cases.iter().enumerate() {
            let result = Solution::max_product(input.clone());
            assert_eq!(
                result, *expected,
                "Test case #{idx}: with nums={input:?}, expected {expected:?}, got {result:?}"
            );
        }
    }
}
