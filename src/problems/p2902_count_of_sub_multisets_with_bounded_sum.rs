//! # LeetCode Problem: 2902 - Count of Sub-Multisets with Bounded Sum
//!
//! Difficulty: Hard
//!
//! Link: https://leetcode.com/problems/count-of-sub-multisets-with-bounded-sum/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(r × unique_numbers)
//! - Space Complexity: O(r)
//!
//! ## A bit of theory
//!
//! We're using Dynamic Programming to solve this problem.
//! In `dp` we store the number of ways to make the sum in a multiset.
//! So, dp[i] = "The number of different ways to select numbers from our
//! array such that their sum equals exactly i"
//!
//! Let's say we have the array [1, 2, 2] and we want sums up to 5.
//! dp[0] = 1    # 1 way to make sum 0: pick nothing {}
//! dp[1] = 1    # 1 way to make sum 1: pick {1}
//! dp[2] = 2    # 2 ways to make sum 2: pick {2} or {2} (we have two 2's)
//! dp[3] = 2    # 2 ways to make sum 3: pick {1,2} or {1,2} (using different 2's)
//! dp[4] = 1    # 1 way to make sum 4: pick {2,2}
//! dp[5] = 1    # 1 way to make sum 5: pick {1,2,2}
//!
//! To solve our problem (count sums between l and r),
//! we add up dp[l] + dp[l+1] + ... + dp[r]
//!
//! This is fundamentally a Bounded Knapsack problem because:
//! - We have items (numbers) with limited quantities (frequencies)
//! - We want to count combinations that achieve certain sums (weights)
//! - Each item can be used 0 to freq times
//!
//! But, for optimum performance, we can use a modified version
//! of the Bounded Knapsack problem.
//! That is, we can use a hybrid version of the Bounded Knapsack problem:
//! Unbounded Knapsack + Bounded Correction.
//!
//! The rationale behind this hybrid is that:
//!
//! Direct Bounded Knapsack would be
//!
//! for count in range(freq + 1):
//!     dp[target] += dp[target - count * num]
//!
//! Time Complexity: O(r × unique_numbers × max_frequency) - potentially too slow!
//!
//! Our Hybrid Approach would be
//! Unbounded: O(r × unique_numbers)
//! Correction: O(r × unique_numbers)
//! Total Time Complexity: O(r × unique_numbers) - much faster!
//!
//! Additional Optimization: Remainder Class Processing
//!
//! for remainder in range(num):
//!
//! This processes positions by their remainder when divided by num,
//! which is a common optimization for knapsack problems with specific item weights.
//!
//! The algorithm breakdown:
//! 1. Handle Zeros Specially
//! 2. Track Maximum Meaningful Sum
//! 3. The Two-Phase Magic
//!    3.1. Phase 1: Add Unlimited Uses (Unbounded Knapsack)
//!    3.2. Phase 2: Remove Excess Uses (Bounded Correction)
//! 4. Count Final Answer
pub struct Solution;

impl Solution {
    pub fn count_sub_multisets(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;

        // Convert to usize for array indexing
        let r_usize = r as usize;

        // 1. Handle Zeros Specially
        // Initialize DP array
        let mut dp = vec![0; r_usize + 1];
        dp[0] = 1; // There is exactly one way to make sum 0. It's an empty multiset.

        // Count frequencies
        let mut counter = std::collections::HashMap::new();
        let mut zero_count = 0;

        for &num in &nums {
            if num == 0 {
                zero_count += 1;
            } else {
                *counter.entry(num).or_insert(0) += 1;
            }
        }

        // Handle zeros - each zero can be included or excluded without affecting the sum
        dp[0] = if zero_count > 0 { zero_count + 1 } else { 1 };

        // 2. Track Maximum Meaningful Sum
        let mut ms: usize = 1;

        for (&num, &freq) in &counter {
            if num > r {
                continue;
            }

            let num_usize = num as usize;

            // Update maximum sum bound
            if ms < dp.len() {
                ms = std::cmp::min(dp.len(), ms + freq as usize * num_usize);
            }

            // 3. The Two-Phase Magic

            // 3.1. Phase 1: Add Unlimited Uses (Unbounded Knapsack)
            for remainder in 0..num_usize {
                let mut k = remainder + num_usize;
                while k < ms {
                    dp[k] = (dp[k] + dp[k - num_usize]) % MOD;
                    k += num_usize;
                }
            }

            // 3.2. Phase 2: Remove Excess Uses (Bounded Correction)
            // Process in reverse order within each remainder class
            for remainder in 0..num_usize {
                // Start from largest k in this remainder class
                let mut k =
                    ((ms - 1).saturating_sub(remainder) / num_usize) * num_usize + remainder;

                while k >= remainder {
                    if (freq as usize + 1) * num_usize > k {
                        break;
                    }
                    // Subtract excess contributions
                    dp[k] = (dp[k] + MOD - dp[k - (freq as usize + 1) * num_usize] % MOD) % MOD;

                    if k < num_usize {
                        break; // Prevent underflow
                    }
                    k -= num_usize;
                }
            }
        }

        // 4. Count Final Answer
        // Sum results in range [l, r]
        let l_usize = l as usize;
        let result = dp
            .iter()
            .skip(l_usize)
            .take(std::cmp::min(r_usize, ms - 1).saturating_sub(l_usize) + 1)
            .fold(0, |acc, &val| (acc + val) % MOD);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_count_sub_multisets() {
        let test_cases = [
            (vec![1, 2, 2, 3], 6, 6, 1),
            (vec![2, 1, 4, 2, 7], 1, 5, 7),
            (vec![1, 2, 1, 3, 5, 2], 3, 5, 9),
        ];
        for (idx, (nums, l, r, expected)) in test_cases.iter().enumerate() {
            let result = Solution::count_sub_multisets(nums.clone(), *l, *r);
            assert_eq!(
                result, *expected,
                "Test case #{idx}: with input {nums:?}, l={l:?}, r={r:?}, expected {expected}, got {result}"
            );
        }
    }
}
