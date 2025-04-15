//! # LeetCode Problem: 2338 - Count the Number of Ideal Arrays
//!
//! Difficulty: Hard
//!
//! Link: https://leetcode.com/problems/count-the-number-of-ideal-arrays/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(max_value * log(max_value) + n * log(max_value))
//! - Space Complexity: O(n) - We use a 2D vector for dynamic programming.
//!
//! ## A bit of theory
//!
//! ### Strictly Increasing Arrays vs. Ideal Arrays
//! In the context of this problem:
//! - An **ideal array** is one where each element (except the first) is divisible
//!         by the previous element. For example, [3, 3, 6, 6] is an ideal array
//!         because each element (except the first) is divisible by the previous element.
//! - A **strictly increasing ideal array** is an ideal array where each element
//!         is strictly greater than the previous element.
//!         For example, [3, 6, 12, 24] is a strictly increasing ideal array.
//!
//! ### Explanation
//! This optimized solution works in multiple steps:
//! 1. **DP for Strictly Increasing Sequences**:
//!         Instead of directly counting all ideal arrays, we first count
//!         strictly increasing ideal arrays.
//!         This significantly reduces the number of combinations we need to consider.
//! 2. **Combinatorial Mathematics**:
//!         The stars and bars theorem from combinatorics helps us calculate how many ways
//!         we can place `n-1` identical objects into `len` distinct boxes, which represents
//!         how many ways we can have a sequence with `len` unique values in an array of length `n`.
//! 3. **Optimization Bounds**:
//!         The maximum number of unique elements in any ideal array is bounded by log(maxValue),
//!         since each next value must be at least twice the previous one.
pub struct Solution;

#[allow(clippy::needless_range_loop)]
impl Solution {
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = n as i64;
        let max_value = max_value as usize;

        // Step 1. First, solve for "strictly increasing" ideal arrays
        //
        // dp[i][j] = number of strictly increasing ideal arrays of length `j` ending with value `i`
        //
        // The vector has at most 15 unique elements in a sequence, because:
        // 1. **Mathematical Constraint**:
        //      In a strictly increasing ideal array, each element must be at least twice
        //      the previous element (since each element must be divisible
        //      by the previous one and be strictly greater). This creates a geometric progression.
        // 2. **Growth Rate Analysis**:
        //      If we start with the smallest possible value 1:
        //      - The sequence would look like: 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048,
        //              4096, 8192, 16384, ...
        //      - By the 15th element, we're already at 16384.
        //              By the definition of the problem, max_value <= 10^4
        // 3. **Maximum Value Bound**:
        //      Since the problem specifies that array elements can't exceed `max_value`,
        //      this geometric growth means that there can't be more than log(max_value)
        //      unique elements in any valid sequence.
        // 4. **Conservative Upper Bound**:
        //      The code uses 16 rows (allowing for sequences with lengths 0 to 15),
        //      which is more than enough for any practical input value of `max_value`
        //      in the problem.
        let mut dp = vec![vec![0; max_value + 1]; 16];

        // Initialize for sequences of length 1
        for i in 1..=max_value {
            dp[1][i] = 1;
        }

        // The minimum growth rate occurs when each element is exactly divisible by the previous
        // one - meaning each element must be at least twice the previous element.
        // For example, starting with 1, the sequence grows: 1, 2, 4, 8, 16, 32, 64, ...
        // This forms a geometric sequence with a ratio of at least 2.
        // The number of terms in such a sequence that don't exceed `max_value` is:
        // 1 + log₂(max_value)
        // Where the +1 accounts for the first element.
        // E.g. for max_value = 100, the sequence would be 1, 2, 4, 8, 16, 32, 64.
        // So, the maximum number of unique elements is 7.
        let max_unique = (max_value as f64).log2().ceil() as usize + 1;
        let max_unique = max_unique.min(15); // Limit to a reasonable number

        // Fill dp for different lengths
        // The code implements the idea that:
        // - If we have a sequence of length `len-1` ending with value `last`
        // - We can extend it to a sequence of length `len` by adding any multiple of `last`
        // - For a strictly increasing sequence, we start with multiples of at least 2 times `last`
        // So if a sequence ends with value 3, the next value could be 6, 9, 12, etc.
        // The code counts how many ways we can build sequences with this rule.
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

        // Step 2. Precompute combinatorial values (for stars and bars)
        // This will store combinatorial values where `comb[i][j]` represents "C(i,j)" or
        // "i choose j" - the number of ways to choose j items from i items
        let mut comb = vec![vec![0i64; max_unique + 1]; (n + 1) as usize];
        for i in 0..=n as usize {
            comb[i][0] = 1;
            for j in 1..=i.min(max_unique) {
                // `comb[i][j] = comb[i-1][j-1] + comb[i-1][j]`
                // is the standard recursive formula for Pascal's triangle
                comb[i][j] = (comb[i - 1][j - 1] + comb[i - 1][j]) % MOD;
            }
        }

        // Step 3. Calculate the final answer using stars and bars method
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
            (900, 10000, 242086103),
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
