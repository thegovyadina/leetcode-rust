//! # LeetCode Problem: 957 - Prison Cells After N Days
//!
//! Difficulty: Medium
//!
//! Link: https://leetcode.com/problems/prison-cells-after-n-days/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(1) - We have a fixed number of cells (8),
//!   and the maximum number of unique states is 64.
//! - Space Complexity: O(1) - We use a limited amount of space for the state map.
//!
//! ## See Also
//! This solution does not use a hash map.
//! It's based on the observation that the prison cells cycle in at most 14-days.
//! https://leetcode.com/problems/prison-cells-after-n-days/solutions/6355350/rust-no-hashmap-solution/

pub struct Solution;

impl Solution {
    pub fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
        if n == 0 {
            return cells;
        }

        // 1. **8 cells but only 6 of them matter**:
        //         In this problem, we have 8 prison cells.
        //         However, due to the rules of cell transformation (where a cell
        //         changes based on its neighbors), the leftmost and rightmost cells
        //         will always become 0 after the first day.
        //         This is because these cells only have one neighbor each, making it impossible
        //         for both adjacent cells to be the same.
        // 2. **Total possible states**:
        //         With 6 middle cells that matter (positions 1-6), and each cell being either 0 or 1,
        //         there are 2^6 = 64 possible unique states.
        // 3. **State transformation is deterministic**:
        //         Given the current state of cells, the rules completely determine the next state.
        //         Each state maps to exactly one next state.
        // 4. **Cycles in state transitions**:
        //         Since there are only 64 possible states, and each state transitions to exactly
        //         one next state, the system must eventually enter a cycle.
        use std::collections::HashMap;

        // Helper function to compute the next state
        fn next_state(cells: &[i32]) -> Vec<i32> {
            let mut next = vec![0; 8];
            for i in 1..7 {
                next[i] = if cells[i - 1] == cells[i + 1] { 1 } else { 0 };
            }
            next
        }

        // Optimization: Convert cell state to a unique u8 for faster hash map lookups
        fn cells_to_u8(cells: &[i32]) -> u8 {
            let mut result = 0u8;
            for (i, &cell) in cells.iter().enumerate() {
                if cell == 1 {
                    result |= 1 << i;
                }
            }
            result
        }

        // Initial transformation for day 1
        let mut cells = next_state(&cells);
        if n == 1 {
            return cells;
        }

        // Find a cycle using a more efficient state map
        let mut state_map: HashMap<u8, i32> = HashMap::new();
        state_map.insert(cells_to_u8(&cells), 1); // Day 1 state

        let mut day = 1;
        while day < n {
            cells = next_state(&cells);
            day += 1;

            let state_key = cells_to_u8(&cells);
            if let Some(prev_day) = state_map.get(&state_key) {
                // We found a cycle
                let cycle_length = day - prev_day;
                let remaining_days = (n - day) % cycle_length;

                // Fast-forward to the final state
                for _ in 0..remaining_days {
                    cells = next_state(&cells);
                }
                return cells;
            }

            state_map.insert(state_key, day);
        }

        cells
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_prison_after_n_days() {
        let test_cases = [
            (
                vec![0, 1, 0, 1, 1, 0, 0, 1],
                7,
                vec![0, 0, 1, 1, 0, 0, 0, 0],
            ),
            (
                vec![1, 0, 0, 1, 0, 0, 1, 0],
                1_000_000_000,
                vec![0, 0, 1, 1, 1, 1, 1, 0],
            ),
            (
                vec![1, 1, 0, 1, 1, 0, 0, 1],
                300_663_720,
                vec![0, 0, 1, 0, 0, 1, 1, 0],
            ),
        ];
        for (idx, (cells, n, expected)) in test_cases.iter().enumerate() {
            let result = Solution::prison_after_n_days(cells.clone(), *n);
            assert_eq!(
                result, *expected,
                "Test case #{idx}: with cells {cells:?}, n={n:?}, expected {expected:?}, got {result:?}"
            );
        }
    }
}
