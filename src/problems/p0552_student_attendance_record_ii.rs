//! # LeetCode Problem: 552 - Student Attendance Record II
//!
//! Difficulty: Hard
//!
//! Link: https://leetcode.com/problems/student-attendance-record-ii/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n) - We use matrix exponentiation to compute the result.
//! - Space Complexity: O(1) - We use a fixed-size matrix for calculations.

pub struct Solution;
pub struct SlowSolution;

impl Solution {
    pub fn check_record(n: i32) -> i32 {
        // Using matrix exponentiation to solve the problem efficiently.
        // The matrix encodes the state transitions for our DP problem.

        // For n=1, the answer is simple: "A", "L", "P" (3 records)
        if n == 1 {
            return 3;
        }

        // Define our transition matrix. Each row represents different states:
        // - Row 0: No 'A', ending with "P"
        // - Row 1: No 'A', ending with "L"
        // - Row 2: No 'A', ending with "LL"
        // - Row 3: Has 'A', ending with "P"
        // - Row 4: Has 'A', ending with "L"
        // - Row 5: Has 'A', ending with "LL"
        let transition_matrix = [
            [0, 0, 1, 0, 0, 0], // Transitions when adding 'P'
            [1, 0, 1, 0, 0, 0], // Transitions when adding 'L'
            [0, 1, 1, 0, 0, 0], // Transitions when adding 'LL'
            [0, 0, 1, 0, 0, 1], // Transitions with 'A' present, adding 'P'
            [0, 0, 1, 1, 0, 1], // Transitions with 'A' present, adding 'L'
            [0, 0, 1, 0, 1, 1], // Transitions with 'A' present, adding 'LL'
        ];

        // We need to compute the state after n+1 days.
        let matrix_power = MatrixPower::new(transition_matrix).pow(n + 1);

        // The answer is at position [5][2] in the matrix
        matrix_power[5][2]
    }
}

// A struct to handle matrix operations efficiently
struct MatrixPower<const N: usize>([[i32; N]; N]);

impl<const N: usize> MatrixPower<N> {
    fn new(matrix: [[i32; N]; N]) -> Self {
        Self(matrix)
    }

    // Multiply two matrices with modular arithmetic
    fn multiply(&self, other: &MatrixPower<N>) -> MatrixPower<N> {
        let mut result = [[0; N]; N];
        const MOD: i64 = 1_000_000_007;

        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    // Use i64 for intermediate calculations to prevent overflow
                    result[i][j] = ((result[i][j] as i64
                        + (self[i][k] as i64 * other[k][j] as i64) % MOD)
                        % MOD) as i32;
                }
            }
        }

        MatrixPower(result)
    }

    // Compute matrix power efficiently using binary exponentiation
    fn pow(self, mut exponent: i32) -> MatrixPower<N> {
        let mut base = self;
        let mut result = identity();

        // Binary exponentiation: M^n = (M^(n/2))^2 if n is even
        // M^n = M * M^(n-1) if n is odd
        while exponent > 0 {
            if exponent % 2 == 1 {
                result = result.multiply(&base);
            }
            base = base.multiply(&base);
            exponent /= 2;
        }

        result
    }
}

/// Returns an identity matrix of size N×N
#[allow(clippy::needless_range_loop)]
fn identity<const N: usize>() -> MatrixPower<N> {
    let mut result = [[0; N]; N];
    for i in 0..N {
        result[i][i] = 1;
    }
    MatrixPower(result)
}

// Implement indexing for our matrix type
impl<const N: usize> std::ops::Index<usize> for MatrixPower<N> {
    type Output = [i32; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

// Implement mutable indexing for our matrix type
impl<const N: usize> std::ops::IndexMut<usize> for MatrixPower<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

/// This is a brute-force solution that generates all possible attendance records.
/// It is not efficient for large inputs but serves as a reference for understanding the problem.
/// It was my first attempt before changing to a more efficient approach.
/// It is not used in the final solution and left here for comparison.
impl SlowSolution {
    pub fn check_record(n: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = n as usize;

        // Initialize DP state arrays with different combinations of 'A' and 'L'
        // dp0 - ending with 'P' with no 'A'
        // dp1 - ending with 'L' (one 'L') with no 'A'
        // dp2 - ending with 'LL' (two 'L's) with no 'A'
        // dp3 - ending with 'P' with one 'A'
        // dp4 - ending with 'L' (one 'L') with one 'A'
        // dp5 - ending with 'LL' (two 'L's) with one 'A'

        // Base cases for n=1
        let mut dp0 = 1; // "P"
        let mut dp1 = 1; // "L"
        let mut dp2 = 0; // no string of length 1 can end with 'LL'
        let mut dp3 = 1; // "A"
        let mut dp4 = 0; // no string of length 1 can be "AL"
        let mut dp5 = 0; // no string of length 1 can be "ALL"

        for _ in 2..=n {
            // Calculate new values
            let new_dp0 = (((dp0 as i64) + (dp1 as i64) + (dp2 as i64)) % (MOD as i64)) as i32;
            let new_dp1 = dp0;
            let new_dp2 = dp1;
            let new_dp3 = (((dp0 as i64)
                + (dp1 as i64)
                + (dp2 as i64)
                + (dp3 as i64)
                + (dp4 as i64)
                + (dp5 as i64))
                % (MOD as i64)) as i32;
            let new_dp4 = dp3;
            let new_dp5 = dp4;

            // Update values for the next iteration
            dp0 = new_dp0;
            dp1 = new_dp1;
            dp2 = new_dp2;
            dp3 = new_dp3;
            dp4 = new_dp4;
            dp5 = new_dp5;
        }

        // Sum all possible endings for the answer
        (((dp0 as i64) + (dp1 as i64) + (dp2 as i64) + (dp3 as i64) + (dp4 as i64) + (dp5 as i64))
            % (MOD as i64)) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::SlowSolution;
    use super::Solution;

    #[test]
    fn test_check_record() {
        let test_cases = [(2, 8), (1, 3), (10101, 183236316)];
        for (idx, (input, expected)) in test_cases.iter().enumerate() {
            let result = Solution::check_record(*input);
            assert_eq!(
                result, *expected,
                "Test case #{}: with input {:?}, expected {:?}, got {:?}",
                idx, input, expected, result
            );
            let result = SlowSolution::check_record(*input);
            assert_eq!(
                result, *expected,
                "Test case for SlowSolution #{}: with input {:?}, expected {:?}, got {:?}",
                idx, input, expected, result
            );
        }
    }
}
