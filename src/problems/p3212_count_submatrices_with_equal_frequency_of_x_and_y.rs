//! # LeetCode Problem: 3212 - Count Submatrices with Equal Frequency of X and Y
//!
//! Difficulty: Medium
//!
//! Link: https://leetcode.com/problems/count-submatrices-with-equal-frequency-of-x-and-y/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n^2 * m) - We iterate through each row and each column, and for each cell,
//!         we perform a constant-time operation to update the dynamic programming state.
//! - Space Complexity: O(m) - We use a vector of size m to store the dynamic programming state.

pub struct Solution;

impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }

        let n = grid[0].len();
        let mut res = 0;
        // dp[i] stores (has_x, running_sum) for each column
        let mut dp = vec![(0, 0); n];

        for row in grid.iter() {
            let mut sum = 0;
            let mut has_x = 0;

            for i in 0..n {
                // Update running sum: +1 for X, -1 for Y
                if row[i] == 'X' {
                    has_x = 1;
                    sum += 1;
                } else if row[i] == 'Y' {
                    sum -= 1;
                }

                // Update dynamic programming state
                // has_x: bitwise OR to track if any X has been seen in this column
                // running_sum: cumulative sum of (X - Y) for each column
                dp[i].0 |= has_x;
                dp[i].1 += sum;

                // If we've seen at least one X and the running sum is 0,
                // we have a submatrix with equal X and Y counts
                if dp[i].0 != 0 && dp[i].1 == 0 {
                    res += 1;
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_number_of_submatrices() {
        let test_cases = [
            (vec![vec!['X', 'Y', '.'], vec!['Y', '.', '.']], 3),
            (vec![vec!['X', 'X'], vec!['X', 'Y']], 0),
            (vec![vec!['.', '.'], vec!['.', '.']], 0),
        ];
        for (idx, (input, expected)) in test_cases.iter().enumerate() {
            let result = Solution::number_of_submatrices(input.clone());
            assert_eq!(
                result, *expected,
                "Test case #{}: with input {:?}, expected {:?}, got {:?}",
                idx, input, expected, result
            );
        }
    }
}
