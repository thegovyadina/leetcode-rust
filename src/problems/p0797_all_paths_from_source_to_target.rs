//! # LeetCode Problem: 797 - All Paths From Source to Target
//!
//! Difficulty: Medium
//!
//! Link: https://leetcode.com/problems/all-paths-from-source-to-target/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(2^n) - In the worst case, we may explore all paths in a complete graph.
//! - Space Complexity: O(n) - The maximum depth of the recursion stack is `n`,
//!         where `n` is the number of nodes in the graph.

pub struct Solution;

impl Solution {
    /// The key point of this solution is to use
    /// [Depth-First Search (DFS)](https://en.wikipedia.org/wiki/Depth-first_search) algorithm
    /// to explore all paths.
    /// DFS is a common algorithm for traversing or searching tree or graph data structures.
    /// It starts at the root (or an arbitrary node) and explores as far as possible along
    /// each branch. Recursion is a common way to implement DFS.
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let target = (graph.len() - 1) as i32;
        let mut path = vec![0];
        Self::dfs(&graph, 0, target, &mut path, &mut result);
        result
    }

    /// Depth-First Search (DFS) to find all paths from the current node to the target node.
    fn dfs(
        graph: &Vec<Vec<i32>>,
        current: i32,
        target: i32,
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        // If the current node is the target, add the path to the result
        if current == target {
            result.push(path.clone());
            return;
        }

        // Explore each neighbor of the current node
        for &next in &graph[current as usize] {
            // Add the next node to the path
            path.push(next);
            // Recursively call DFS for the next node
            Self::dfs(graph, next, target, path, result);
            // Backtrack: remove the last node from the path
            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_all_paths_source_target() {
        let test_cases = [
            (
                vec![vec![1, 2], vec![3], vec![3], vec![]],
                vec![vec![0, 1, 3], vec![0, 2, 3]],
            ),
            (
                vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]],
                vec![
                    vec![0, 4],
                    vec![0, 3, 4],
                    vec![0, 1, 3, 4],
                    vec![0, 1, 2, 3, 4],
                    vec![0, 1, 4],
                ],
            ),
        ];
        for (idx, (input, expected)) in test_cases.iter().enumerate() {
            let result = Solution::all_paths_source_target(input.clone());
            assert_eq!(
                result, *expected,
                "Test case #{}: with input {:?}, expected {:?}, got {:?}",
                idx, input, expected, result
            );
        }
    }
}
