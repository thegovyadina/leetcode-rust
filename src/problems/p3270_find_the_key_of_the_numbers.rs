//! # LeetCode Problem: 3270 - Find the Key of the Numbers
//!
//! Difficulty: Easy
//!
//! Link: https://leetcode.com/problems/find-the-key-of-the-numbers/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(1) - We iterate through the three numbers a constant number of times.
//! - Space Complexity: O(1) - We use a constant amount of space for the result.

pub struct Solution;

impl Solution {
    pub fn generate_key(num1: i32, num2: i32, num3: i32) -> i32 {
        let mut thousands = 9;
        let mut hundreds = 9;
        let mut tens = 9;
        let mut units = 9;
        for &n in &[num1, num2, num3] {
            thousands = thousands.min(n / 1000 % 10);
            hundreds = hundreds.min(n / 100 % 10);
            tens = tens.min(n / 10 % 10);
            units = units.min(n % 10);
        }
        thousands * 1000 + hundreds * 100 + tens * 10 + units
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_generate_key() {
        let test_cases = [(1, 10, 1000, 0), (987, 879, 798, 777), (1, 2, 3, 1)];
        for (idx, (input1, input2, input3, expected)) in test_cases.iter().enumerate() {
            let result = Solution::generate_key(*input1, *input2, *input3);
            assert_eq!(
                result, *expected,
                "Test case #{}: with input1 {:?}, {:?}, {:?}, expected {:?}, got {:?}",
                idx, input1, input2, input3, expected, result
            );
        }
    }
}
