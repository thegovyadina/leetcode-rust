//! # LeetCode Problem: 707 - Design Linked List
//!
//! Difficulty: Medium
//!
//! Link: https://leetcode.com/problems/design-linked-list/

#![allow(dead_code)]

struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

struct MyLinkedList {
    head: Option<Box<Node>>,
    size: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList {
            head: None,
            size: 0,
        }
    }

    fn get(&self, index: i32) -> i32 {
        if index < 0 || index as usize >= self.size {
            return -1;
        }

        let mut current = &self.head;
        for _ in 0..index {
            if let Some(node) = current {
                current = &node.next;
            } else {
                return -1;
            }
        }

        match current {
            Some(node) => node.val,
            None => -1,
        }
    }

    fn add_at_head(&mut self, val: i32) {
        let new_node = Box::new(Node {
            val,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    fn add_at_tail(&mut self, val: i32) {
        let new_node = Box::new(Node { val, next: None });

        if self.head.is_none() {
            self.head = Some(new_node);
        } else {
            let mut current = &mut self.head;
            while let Some(node) = current {
                if node.next.is_none() {
                    node.next = Some(new_node);
                    break;
                }
                current = &mut node.next;
            }
        }
        self.size += 1;
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index < 0 || index as usize > self.size {
            return;
        }

        if index == 0 {
            self.add_at_head(val);
            return;
        }

        let mut current = &mut self.head;
        for _ in 0..index - 1 {
            if let Some(node) = current {
                current = &mut node.next;
            } else {
                return;
            }
        }

        if let Some(node) = current {
            let new_node = Box::new(Node {
                val,
                next: node.next.take(),
            });
            node.next = Some(new_node);
            self.size += 1;
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        if index < 0 || index as usize >= self.size {
            return;
        }

        if index == 0 {
            if let Some(node) = self.head.take() {
                self.head = node.next;
                self.size -= 1;
            }
            return;
        }

        let mut current = &mut self.head;
        for _ in 0..index - 1 {
            if let Some(node) = current {
                current = &mut node.next;
            } else {
                return;
            }
        }

        if let Some(node) = current {
            if let Some(next_node) = node.next.take() {
                node.next = next_node.next;
                self.size -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MyLinkedList;

    #[test]
    fn test_my_linked_list() {
        let mut linked_list = MyLinkedList::new();
        linked_list.add_at_head(1);
        linked_list.add_at_tail(3);
        linked_list.add_at_index(1, 2);
        assert_eq!(linked_list.get(1), 2);
        linked_list.delete_at_index(1);
        assert_eq!(linked_list.get(1), 3);
    }
}
