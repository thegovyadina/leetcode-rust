//! # LeetCode Problem: 2471 - Minimum Number of Operations to Sort a Binary Tree by Level
//!
//! Difficulty: Medium
//!
//! Link: https://leetcode.com/problems/minimum-number-of-operations-to-sort-a-binary-tree-by-level/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n * log(n)) - We perform a level order traversal of the tree,
//!   and for each level, we sort the values.
//! - Space Complexity: O(n) - We use a queue to perform level order traversal.

pub struct Solution;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl Solution {
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // We'll go level by level and sort the values at each of them.
        // In `queue` we'll store the nodes at the current level as we traverse.
        let mut queue = VecDeque::from([root.clone().unwrap()]);
        // In `order` we'll store the indices of the nodes at the current level.
        let mut order = Vec::new();
        let mut swap_count = 0;

        while !queue.is_empty() {
            // Perform level order traversal.
            // We need to sort the values at this level.
            order.extend(0..queue.len());
            order.sort_unstable_by_key(|&i| queue[i].borrow().val);

            // Count the number of swaps needed to sort the values at this level
            for i in 0..order.len() {
                while order[i] != i {
                    let j = order[i];
                    order.swap(i, j);
                    swap_count += 1;
                }
            }

            // Cycle over the nodes at this level and add their children to the queue
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                let node = node.borrow();
                if let Some(left) = node.left.as_ref() {
                    queue.push_back(left.clone());
                }
                if let Some(right) = node.right.as_ref() {
                    queue.push_back(right.clone());
                }
            }
            // Clear the order for the next level
            order.clear();
        }
        swap_count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use super::TreeNode;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    fn build_tree_from_level_order(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() || values[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&root));

        let mut i = 1;
        while !queue.is_empty() && i < values.len() {
            let node = queue.pop_front().unwrap();

            // Left child
            if i < values.len() && values[i].is_some() {
                let left = Rc::new(RefCell::new(TreeNode::new(values[i].unwrap())));
                node.borrow_mut().left = Some(Rc::clone(&left));
                queue.push_back(left);
            }
            i += 1;

            // Right child
            if i < values.len() && values[i].is_some() {
                let right = Rc::new(RefCell::new(TreeNode::new(values[i].unwrap())));
                node.borrow_mut().right = Some(Rc::clone(&right));
                queue.push_back(right);
            }
            i += 1;
        }

        Some(root)
    }

    #[test]
    fn test_minimum_operations() {
        let test_cases: [(Vec<Option<i32>>, i32); 3] = [
            (
                vec![
                    Some(1),
                    Some(4),
                    Some(3),
                    Some(7),
                    Some(6),
                    Some(8),
                    Some(5),
                    None,
                    None,
                    None,
                    None,
                    Some(9),
                    None,
                    Some(10),
                ],
                3,
            ),
            (
                vec![
                    Some(1),
                    Some(3),
                    Some(2),
                    Some(7),
                    Some(6),
                    Some(5),
                    Some(4),
                ],
                3,
            ),
            (
                vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)],
                0,
            ),
        ];
        for (idx, (input, expected)) in test_cases.iter().enumerate() {
            let tree = build_tree_from_level_order(input.clone());
            let result = Solution::minimum_operations(tree);
            assert_eq!(
                result, *expected,
                "Test case #{idx}: with input {input:?}, expected {expected}, got {result}"
            );
        }
    }
}
