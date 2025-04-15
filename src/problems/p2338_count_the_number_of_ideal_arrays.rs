//! # LeetCode Problem: 2338 - Count the Number of Ideal Arrays
//!
//! Difficulty: Hard
//!
//! Link: https://leetcode.com/problems/count-the-number-of-ideal-arrays/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n * log(max_value)) - We iterate through the numbers from 1 to max_value.
//! - Space Complexity: O(n) - We use a 2D vector for dynamic programming.

pub struct Solution;

impl Solution {
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = n as i64;
        let max_value = max_value as usize;

        // 1. First, solve for "strictly increasing" ideal arrays
        // dp[i] = number of strictly increasing ideal arrays ending with value i
        let mut dp = vec![vec![0; max_value + 1]; 16]; // At most 15 unique elements in a sequence

        // Initialize for sequences of length 1
        for i in 1..=max_value {
            dp[1][i] = 1;
        }

        // Maximum unique elements possible is log(max_value)
        let max_unique = (max_value as f64).log2().ceil() as usize + 1;
        let max_unique = max_unique.min(15); // Limit to a reasonable number

        // Fill dp for different lengths
        for len in 2..=max_unique {
            for last in 1..=max_value {
                // For each divisor of last, we can extend the sequence
                let mut j = last * 2; // Start from last * 2 for strictly increasing
                while j <= max_value {
                    dp[len][j] = (dp[len][j] + dp[len - 1][last]) % MOD;
                    j += last;
                }
            }
        }

        // 2. Precompute combinatorial values (for stars and bars)
        let mut comb = vec![vec![0i64; max_unique + 1]; (n + 1) as usize];
        // comb[n][k] = C(n, k) = number of ways to choose k items from n items
        for i in 0..=n as usize {
            comb[i][0] = 1;
            for j in 1..=i.min(max_unique) {
                comb[i][j] = (comb[i - 1][j - 1] + comb[i - 1][j]) % MOD;
            }
        }

        // 3. Calculate the final answer using stars and bars method
        let mut result = 0i64;
        for len in 1..=max_unique {
            let mut count = 0i64;
            // Sum up all strictly increasing sequences of length len
            for last in 1..=max_value {
                count = (count + dp[len][last]) % MOD;
            }

            // Using stars and bars: ways to place n-1 balls into len slots
            // C(n-1, len-1) = number of ways to have len unique elements in array of length n
            if n > 1 {
                count = (count * comb[(n - 1) as usize][len - 1]) % MOD;
            }

            result = (result + count) % MOD;
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_ideal_arrays() {
        let test_cases = [
            (2, 5, 10),
            (5, 3, 11),
            (435, 34, 603467738),
            (5878, 2900, 465040898),
        ];
        for (idx, (input_n, input_max, expected)) in test_cases.iter().enumerate() {
            let result = Solution::ideal_arrays(*input_n, *input_max);
            assert_eq!(
                result, *expected,
                "Test case #{}: with input string {:?} and distance {:?}, expected {:?}, got {:?}",
                idx, input_n, input_max, expected, result
            );
        }
    }
}
