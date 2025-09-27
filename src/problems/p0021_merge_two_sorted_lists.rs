//! # LeetCode Problem: 21 - Merge Two Sorted Lists
//!
//! Difficulty: Easy
//!
//! Link: https://leetcode.com/problems/merge-two-sorted-lists/
//!
//! ## Complexity Analysis
//! - Time Complexity: O(n) - We traverse both lists exactly once.
//! - Space Complexity: O(1) - We use a constant amount of extra space
//!   regardless of the input size.
pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy_head = ListNode::new(0);
        let mut current = &mut dummy_head;

        let mut l1 = list1;
        let mut l2 = list2;

        while l1.is_some() && l2.is_some() {
            if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
                current.next = l1;
                current = current.next.as_mut().unwrap();
                l1 = current.next.take();
            } else {
                current.next = l2;
                current = current.next.as_mut().unwrap();
                l2 = current.next.take();
            }
        }

        if l1.is_some() {
            current.next = l1;
        } else {
            current.next = l2;
        }

        dummy_head.next
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};

    // Helper function to convert a vector to a linked list
    fn vec_to_list(vec: &[i32]) -> Option<Box<ListNode>> {
        let mut dummy_head = None;
        // We need to build the list in reverse order
        for &val in vec.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = dummy_head;
            dummy_head = Some(Box::new(node));
        }
        dummy_head
    }

    // Helper function to convert a linked list to a vector
    fn list_to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = list;
        while let Some(node) = current {
            result.push(node.val);
            current = node.next;
        }
        result
    }

    #[test]
    fn test_merge_two_lists() {
        let test_cases = [
            (vec![1, 2, 4], vec![1, 3, 4], vec![1, 1, 2, 3, 4, 4]),
            (vec![], vec![], vec![]),
            (vec![], vec![0], vec![0]),
        ];

        for (idx, (input1, input2, expected)) in test_cases.iter().enumerate() {
            let list1 = vec_to_list(input1);
            let list2 = vec_to_list(input2);
            let result = Solution::merge_two_lists(list1, list2);

            let result_vec = list_to_vec(result);
            assert_eq!(
                result_vec, *expected,
                "Test case #{idx}: with input1 {input1:?}, input2 {input2:?}, expected {expected:?}, got {result_vec:?}"
            );
        }
    }
}
