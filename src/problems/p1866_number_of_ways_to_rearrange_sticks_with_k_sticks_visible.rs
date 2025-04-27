//! # LeetCode Problem: 1866 - Number of Ways to Rearrange Sticks with K Sticks Visible
//!
//! Difficulty: Hard
//!
//! Link: https://leetcode.com/problems/number-of-ways-to-rearrange-sticks-with-k-sticks-visible/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n * k) - We use a nested loop to calculate the number of ways.
//! - Space Complexity: O(n * k) - We use a 2D vector to store the number of ways for each n and k.

pub struct Solution;

impl Solution {
    pub fn rearrange_sticks(n: i32, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = n as usize;
        let k = k as usize;

        let mut dp = vec![vec![0i64; k + 1]; n + 1];
        dp[0][0] = 1;

        for i in 1..=n {
            for j in 1..=k.min(i) {
                // The number of ways to arrange i sticks with j visible
                // is the sum of the number of ways to arrange (i-1) sticks
                // with (j-1) visible and the number of ways to arrange (i-1) sticks
                // with j visible, multiplied by (i-1) because we can place the new stick
                // in any of the (i-1) positions.
                // Convert to i64 to avoid overflow during multiplication.
                dp[i][j] = (dp[i - 1][j - 1] + ((i - 1) as i64 * dp[i - 1][j])) % MOD;
            }
        }

        dp[n][k] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_rearrange_sticks() {
        let test_cases = [(3, 2, 3), (5, 5, 1), (20, 11, 647427950)];
        for (idx, (n, k, expected)) in test_cases.iter().enumerate() {
            let result = Solution::rearrange_sticks(*n, *k);
            assert_eq!(
                result, *expected,
                "Test case #{}: with n={:?}, k={:?}, expected {:?}, got {:?}",
                idx, n, k, expected, result
            );
        }
    }
}
