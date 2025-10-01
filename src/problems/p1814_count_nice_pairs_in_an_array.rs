//! # LeetCode Problem: 1814 - Count Nice Pairs in an Array
//!
//! ## Description
//!
//! Difficulty: Medium
//!
//! Link: https://leetcode.com/problems/count-nice-pairs-in-an-array/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n) - We traverse the list containing n elements exactly twice.
//! - Space Complexity: O(n) - We use a HashMap to store the frequency of each modified number.

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        // For comparison of the loop-based and string-based reverting functions,
        // see benchmark /benches/p1814_benchmark.rs
        // You can execute the benchmarks by running `cargo bench`
        fn rev(mut num: i32) -> i32 {
            let mut result = 0;
            while num > 0 {
                result = result * 10 + num % 10;
                num /= 10;
            }
            result
        }

        // Count frequencies of differences in one pass
        let mut diff_counts: HashMap<i32, i64> = HashMap::new();
        for num in nums {
            let diff = num - rev(num);
            *diff_counts.entry(diff).or_insert(0) += 1;
        }
        // Calculate combinations: n choose 2 = n * (n - 1) / 2
        let mut result: i64 = 0;
        for &count in diff_counts.values() {
            if count >= 2 {
                result = (result + (count * (count - 1) / 2)) % MOD;
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_count_nice_pairs() {
        let test_cases = [(vec![42, 11, 1, 97], 2), (vec![13, 10, 35, 24, 76], 4)];
        for (idx, (nums, expected)) in test_cases.iter().enumerate() {
            let result = Solution::count_nice_pairs(nums.clone());
            assert_eq!(
                result, *expected,
                "Test case #{idx}: with input {nums:?}, expected {expected:?}, got {result:?}"
            );
        }
    }
}
