//! # LeetCode Problem: 2933 - High Access Employees
//!
//! Difficulty: Medium
//!
//! Link: https://leetcode.com/problems/high-access-employees/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n log n)
//! - Space Complexity: O(n) - We use a hash map to store the access times for each employee.

pub struct Solution;

impl Solution {
    pub fn find_high_access_employees(access_times: Vec<Vec<String>>) -> Vec<String> {
        use std::collections::HashMap;
        let mut access_map: HashMap<String, Vec<i32>> = HashMap::new();

        // Collect all access times for each employee
        for access in access_times {
            let employee = &access[0];
            let time = access[1].parse::<i32>().unwrap();
            access_map.entry(employee.clone()).or_default().push(time);
        }

        let mut result: Vec<String> = vec![];

        // Process each employee's access times
        for (employee, times) in access_map.iter_mut() {
            if times.len() < 3 {
                continue; // Skip employees with fewer than 3 accesses
            }

            // Sort the access times
            times.sort_unstable();

            // Check for 3 accesses within a one-hour period
            let mut is_high_access = false;
            for i in 0..times.len() - 2 {
                // Convert from minutes to check if within one hour
                // A one-hour period is 60 minutes = 100 in this time format
                if times[i + 2] - times[i] < 100 {
                    is_high_access = true;
                    break;
                }
            }

            if is_high_access {
                result.push(employee.clone());
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::collections::HashSet;

    #[test]
    fn test_find_high_access_employees() {
        let test_cases = [
            (
                vec![
                    vec!["a", "0549"],
                    vec!["b", "0457"],
                    vec!["a", "0532"],
                    vec!["a", "0621"],
                    vec!["b", "0540"],
                ],
                vec!["a"],
            ),
            (
                vec![
                    vec!["d", "0002"],
                    vec!["c", "0808"],
                    vec!["c", "0829"],
                    vec!["e", "0215"],
                    vec!["d", "1508"],
                    vec!["d", "1444"],
                    vec!["d", "1410"],
                    vec!["c", "0809"],
                ],
                vec!["c", "d"],
            ),
            (
                vec![
                    vec!["cd", "1025"],
                    vec!["ab", "1025"],
                    vec!["cd", "1046"],
                    vec!["cd", "1055"],
                    vec!["ab", "1124"],
                    vec!["ab", "1120"],
                ],
                vec!["ab", "cd"],
            ),
            (
                vec![
                    vec!["akuhmu", "0454"],
                    vec!["aywtqh", "0523"],
                    vec!["akuhmu", "0518"],
                    vec!["ihhkc", "0439"],
                    vec!["ihhkc", "0508"],
                    vec!["akuhmu", "0529"],
                    vec!["aywtqh", "0530"],
                    vec!["aywtqh", "0419"],
                ],
                vec!["akuhmu"],
            ),
        ];
        for (idx, (input, expected)) in test_cases.iter().enumerate() {
            // Convert the input and expected values to String
            let string_input: Vec<Vec<String>> = input
                .iter()
                .map(|v| v.iter().map(|s| s.to_string()).collect())
                .collect();

            let string_expected: Vec<String> = expected.iter().map(|s| s.to_string()).collect();
            let result = Solution::find_high_access_employees(string_input);
            // Sort both vectors to normalize them (not strictly necessary but helps with debugging)
            let mut sorted_result = result.clone();
            let mut sorted_expected = string_expected.clone();
            sorted_result.sort();
            sorted_expected.sort();
            assert_eq!(
                sorted_result.iter().collect::<HashSet<_>>(),
                sorted_expected.iter().collect::<HashSet<_>>(),
                "Test case #{idx}: with input {input:?}, expected {expected:?}, got {result:?}"
            );
        }
    }
}
