//! # LeetCode Problem: 39 - Combination Sum
//!
//! Difficulty: Medium
//!
//! Link: https://leetcode.com/problems/combination-sum/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(2^n) - The number of combinations can be exponential in the worst case.
//! - Space Complexity: O(n) - The space used by the recursion stack and the result list.
pub struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut current_combination = vec![];
        Self::backtrack(
            &candidates,
            target,
            0,
            &mut current_combination,
            &mut result,
        );
        result
    }
    fn backtrack(
        candidates: &[i32],
        target: i32,
        start: usize,
        current_combination: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            result.push(current_combination.clone());
            return;
        }
        for i in start..candidates.len() {
            if candidates[i] > target {
                continue;
            }
            current_combination.push(candidates[i]);
            Self::backtrack(
                candidates,
                target - candidates[i],
                i,
                current_combination,
                result,
            );
            current_combination.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_combination_sum() {
        let test_cases = [
            (vec![2, 3, 6, 7], 7, vec![vec![2, 2, 3], vec![7]]),
            (
                vec![2, 3, 5],
                8,
                vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]],
            ),
            (vec![2], 1, vec![]),
        ];
        for (idx, (input, n, expected)) in test_cases.iter().enumerate() {
            let result = Solution::combination_sum(input.clone(), *n);
            assert_eq!(
                result, *expected,
                "Test case #{}: with input {:?}, expected {:?}, got {:?}",
                idx, input, expected, result
            );
        }
    }
}
