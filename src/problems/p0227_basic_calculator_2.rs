//! # LeetCode Problem: 227 - Basic Calculator II
//!
//! Difficulty: Medium
//!
//! Link: https://leetcode.com/problems/basic-calculator-ii/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n) - We iterate through the string once, processing each character.
//! - Space Complexity: O(n) - We use a stack to store intermediate results,
//!   which can grow linearly with the input size.

pub struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack: Vec<i32> = vec![];
        let mut current_number = 0;
        let mut operation = '+';

        for (i, c) in s.chars().enumerate() {
            let is_digit = c.is_ascii_digit();
            if is_digit {
                // Multiply the current number by 10 and add the new digit
                // to handle multi-digit numbers
                current_number = current_number * 10 + c.to_digit(10).unwrap() as i32;
            }

            if !is_digit && c != ' ' || i == s.len() - 1 {
                match operation {
                    '+' => stack.push(current_number),
                    '-' => stack.push(-current_number),
                    '*' => {
                        let last = stack.pop().unwrap();
                        stack.push(last * current_number);
                    }
                    '/' => {
                        let last = stack.pop().unwrap();
                        stack.push(last / current_number);
                    }
                    _ => {}
                }
                operation = c;
                current_number = 0;
            }
        }

        stack.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_calculate() {
        let test_cases = [
            ("3+2*2", 7),
            (" 3/2 ", 1),
            (" 3+5 / 2 ", 5),
            ("73+5*1-8/3", 76),
            ("2700- 5*2*5*3", 2550),
            ("-4+0/2  ", -4),
            ("-3*2*108/2/4", -81),
            ("3+5*2-18 / 4+13411", 13420),
        ];
        for (idx, (input, expected)) in test_cases.iter().enumerate() {
            let result = Solution::calculate(input.to_string());
            assert_eq!(
                result, *expected,
                "Test case #{idx}: with input {input:?}, expected {expected:?}, got {result:?}"
            );
        }
    }
}
