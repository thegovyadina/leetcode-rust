//! # LeetCode Problem: 2442 - Count Number of Distinct Integers After Reverse Operations
//!
//! Difficulty: Medium
//!
//! Link: https://leetcode.com/problems/count-number-of-distinct-integers-after-reverse-operations/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n) - We iterate through the list of integers once.
//! - Space Complexity: O(n) - We use a HashSet to store distinct integers.

pub struct Solution;

impl Solution {
    // This is my first attempt at solving the problem.
    // Converting an integer to string, reverting it and parsing back to integer
    // appeared to be too slow.
    // I leave it here for comparison purposes.

    // pub fn count_distinct_integers(nums: Vec<i32>) -> i32 {
    //     let mut distinct_integers = std::collections::HashSet::new();
    //     for &num in &nums {
    //         distinct_integers.insert(num);
    //         let reversed_num = num.to_string().chars().rev().collect::<String>();
    //         if let Ok(reversed_num) = reversed_num.parse::<i32>() {
    //             distinct_integers.insert(reversed_num);
    //         }
    //     }
    //     distinct_integers.len() as i32
    // }

    pub fn count_distinct_integers(nums: Vec<i32>) -> i32 {
        let mut distinct_integers = std::collections::HashSet::new();

        for &num in &nums {
            distinct_integers.insert(num);

            let mut n = num;
            let mut reversed = 0;

            while n != 0 {
                reversed = reversed * 10 + n % 10;
                n /= 10;
            }

            distinct_integers.insert(reversed);
        }

        distinct_integers.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_count_distinct_integers() {
        let test_cases = [(vec![1, 13, 10, 12, 31], 6), (vec![2, 2, 2], 1)];
        for (idx, (input, expected)) in test_cases.iter().enumerate() {
            let result = Solution::count_distinct_integers(input.clone());
            assert_eq!(
                result, *expected,
                "Test case #{}: with input {:?}, expected {:?}, got {:?}",
                idx, input, expected, result
            );
        }
    }
}
