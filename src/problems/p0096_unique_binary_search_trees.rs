//! # LeetCode Problem: 96 - Unique Binary Search Trees
//!
//! Difficulty: Medium
//!
//! Link: https://leetcode.com/problems/unique-binary-search-trees/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n^2) - We use a nested loop to calculate the number of unique BSTs.
//! - Space Complexity: O(n) - We use a vector to store the number of unique BSTs
//!   for each number of nodes.
pub struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        // The number of unique BSTs with n nodes is given by the nth Catalan number
        // C(n) = (2n)! / ((n + 1)! * n!)
        // We can use dynamic programming to calculate the nth Catalan number
        let mut dp = vec![0; (n + 1) as usize];
        dp[0] = 1;
        dp[1] = 1;

        for i in 2..=n {
            for j in 0..i {
                dp[i as usize] += dp[j as usize] * dp[(i - 1 - j) as usize];
            }
        }

        dp[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_num_trees() {
        let test_cases = [(3, 5), (1, 1)];
        for (idx, (input, expected)) in test_cases.iter().enumerate() {
            let result = Solution::num_trees(*input);
            assert_eq!(
                result, *expected,
                "Test case #{idx}: with input {input:?}, expected {expected:?}, got {result:?}"
            );
        }
    }
}
