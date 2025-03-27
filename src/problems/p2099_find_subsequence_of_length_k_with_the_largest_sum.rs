//! # LeetCode Problem: 2099 - Find Subsequence of Length K With the Largest Sum
//!
//! Difficulty: Easy
//!
//! Link: https://leetcode.com/problems/find-subsequence-of-length-k-with-the-largest-sum/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n log n) - We sort the array of size n.
//! - Space Complexity: O(n) - We use a vector to store the indices of the top k elements.

pub struct Solution;

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;

        // Create pairs of (index, value) for tracking original positions
        let mut indexed_nums: Vec<(usize, i32)> =
            nums.iter().enumerate().map(|(i, &val)| (i, val)).collect();

        // Sort by value in descending order
        indexed_nums.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        // Take `k` largest elements
        let top_k_indices: Vec<usize> = indexed_nums.iter().take(k).map(|&(idx, _)| idx).collect();

        // Sort indices to maintain original order
        let mut result_indices = top_k_indices;
        result_indices.sort_unstable();

        // Return elements in their original order
        result_indices.iter().map(|&idx| nums[idx]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_subsequence() {
        let test_cases = [
            (vec![2, 1, 3, 3], 2, vec![3, 3]),
            (vec![-1, -2, 3, 4], 3, vec![-1, 3, 4]),
            (vec![3, 4, 3, 3], 2, vec![3, 4]),
        ];
        for (idx, (nums, k, expected)) in test_cases.iter().enumerate() {
            let result = Solution::max_subsequence(nums.clone(), *k);
            assert_eq!(
                result, *expected,
                "Test case #{}: with nums={:?} and k={:?}, expected {:?}, got {:?}",
                idx, nums, k, expected, result
            );
        }
    }
}
