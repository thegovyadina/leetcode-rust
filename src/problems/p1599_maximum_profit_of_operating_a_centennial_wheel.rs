//! # LeetCode Problem: 1599 - Maximum Profit of Operating a Centennial Wheel
//!
//! Difficulty: Medium
//!
//! Link: https://leetcode.com/problems/maximum-profit-of-operating-a-centennial-wheel/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n) - We iterate through the list of customers once,
//! - Space Complexity: O(1) - We use a constant amount of space for the result.
//!
//! ## A brief explanation of the solution
//! This is one more problem where going through the extremely misleading description
//! is the most challenging part.
//! The idea is to simulate the process of boarding customers on a wheel.
//! We keep track of the number of customers waiting to board, the total profit,
//! and the maximum profit seen so far.
//! We iterate through the list of customers, boarding up to 4 customers at a time
//! and calculating the profit for each rotation.
//! If the profit is greater than the maximum profit seen so far,
//! we update the maximum profit and the rotation at which it was achieved.
pub struct Solution;

impl Solution {
    pub fn min_operations_max_profit(
        customers: Vec<i32>,
        boarding_cost: i32,
        running_cost: i32,
    ) -> i32 {
        // If boarding 4 customers costs less than a single rotation, we can never make a profit
        if boarding_cost * 4 <= running_cost {
            return -1;
        }

        let mut waiting = 0; // Customers waiting to board
        let mut total_profit = 0; // Current profit
        let mut max_profit = 0; // Maximum profit seen so far
        let mut total_rotations = 0; // Total number of rotations made
        let mut max_profit_rotation = -1; // Rotation at which max profit was achieved

        // Process all customer arrivals
        for &new_customers in customers.iter() {
            waiting += new_customers;

            // Board up to 4 customers
            let boarding = waiting.min(4);
            waiting -= boarding;

            // Calculate profit for this rotation
            total_rotations += 1;
            total_profit += boarding * boarding_cost - running_cost;

            // Update max profit if the current profit is better
            if total_profit > max_profit {
                max_profit = total_profit;
                max_profit_rotation = total_rotations;
            }
        }

        // Continue rotating the wheel while there are waiting customers
        while waiting > 0 {
            // Board up to 4 customers
            let boarding = waiting.min(4);
            waiting -= boarding;

            // Calculate profit for this rotation
            total_rotations += 1;
            total_profit += boarding * boarding_cost - running_cost;

            // Update max profit if the current profit is better
            if total_profit > max_profit {
                max_profit = total_profit;
                max_profit_rotation = total_rotations;
            }
        }

        // Return -1 if we can't make a profit
        if max_profit <= 0 {
            return -1;
        }

        max_profit_rotation
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_min_operations_max_profit() {
        let test_cases = [
            (vec![8, 3], 5, 6, 3),
            (vec![10, 9, 6], 6, 4, 7),
            (vec![3, 4, 0, 5, 1], 1, 92, -1),
        ];
        for (idx, (customers, boarding_cost, running_cost, expected)) in
            test_cases.iter().enumerate()
        {
            let result = Solution::min_operations_max_profit(
                customers.clone(),
                *boarding_cost,
                *running_cost,
            );
            assert_eq!(
                result, *expected,
                "Test case #{idx}: with customers {customers:?}, boarding_cost={boarding_cost:?}, running_cost={running_cost:?}, expected {expected:?}, got {result:?}"
            );
        }
    }
}
