#![allow(dead_code)]

use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    fmt::Display,
    ops::BitXorAssign,
    rc::{Rc, Weak},
    result,
};

use crate::algo::recursion;

// TODO: Make a binary tree based on an array ?

// Maybe over engineered, Box<T> should be Ok for a Binary Tree
type LinkedBinaryTreeNodeRef<T> = Rc<RefCell<LinkedBinaryTreeNode<T>>>;

pub struct LinkedBinaryTree<T: Display> {
    root: Option<LinkedBinaryTreeNodeRef<T>>,
}

impl<T: Display> LinkedBinaryTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    fn count(&self) -> usize {
        if let Some(root) = &self.root {
            root.borrow().count()
        } else {
            0
        }
    }

    fn is_full(&self) -> bool {
        if let Some(root) = &self.root {
            root.borrow().is_full()
        } else {
            false
        }
    }

    fn is_perfect(&self) -> bool {
        todo!()
    }

    fn is_complete(&self) -> bool {
        if let Some(root) = &self.root {
            root.borrow().is_complete()
        } else {
            false
        }
    }

    fn is_balanced(&self) -> bool {
        todo!()
    }

    // TODO try to implement in iteration with a stack variable.
    fn traverse_pre_order(&self, result_recip: &mut String) {
        if let Some(root) = &self.root {
            root.borrow().traverse_pre_order(result_recip);
        }
    }

    // TODO try to implement in iteration with a stack variable.
    fn traverse_in_order(&self, result_recip: &mut String) {
        if let Some(root) = &self.root {
            root.borrow().traverse_in_order(result_recip);
        }
    }

    // TODO try to implement in iteration with a stack variable.
    fn traverse_post_order(&self, result_recip: &mut String) {
        if let Some(root) = &self.root {
            root.borrow().traverse_post_order(result_recip);
        }
    }

    fn traverse_level_order(&self, result_recip: &mut String) {
        // TODO: implement method len on the tree, to then use ArrayQueue.
        let mut queue = VecDeque::with_capacity(10);
        if let Some(root) = &self.root {
            result_recip.push_str(format!("{} ", root.borrow().val).as_str());
            queue.push_back(root.clone());
            while !queue.is_empty() {
                let node_ref = queue.pop_front().unwrap();
                if let Some(ref left) = node_ref.clone().borrow().left {
                    result_recip.push_str(format!("{} ", left.borrow().val).as_str());
                    queue.push_back(left.clone());
                }
                if let Some(ref right) = node_ref.clone().borrow().right {
                    result_recip.push_str(format!("{} ", right.borrow().val).as_str());
                    queue.push_back(right.clone());
                }
            }
        }
    }
}

// TODO: try to implement Iter using Stack or DFS cases and Queue for BFS

struct LinkedBinaryTreeNode<T: Display> {
    val: T,
    left: Option<LinkedBinaryTreeNodeRef<T>>,
    right: Option<LinkedBinaryTreeNodeRef<T>>,
}

impl<T: Display> LinkedBinaryTreeNode<T> {
    pub fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn count(&self) -> usize {
        let left_count = if let Some(left) = &self.left {
            left.borrow().count()
        } else {
            0
        };

        let right_count = if let Some(right) = &self.right {
            right.borrow().count()
        } else {
            0
        };

        left_count + right_count + 1
    }

    pub fn set_left(&mut self, val: T) {
        self.left = Some(Rc::new(RefCell::new(LinkedBinaryTreeNode::new(val))));
    }

    pub fn set_right(&mut self, val: T) {
        self.right = Some(Rc::new(RefCell::new(LinkedBinaryTreeNode::new(val))));
    }

    pub fn is_full(&self) -> bool {
        match (&self.left, &self.right) {
            (Some(left), Some(right)) => left.borrow().is_full() && right.borrow().is_full(),
            (None, None) => true,
            _ => false,
        }
    }

    pub fn is_complete(&self) -> bool {
        match (&self.left, &self.right) {
            (Some(left), Some(right)) => {
                left.borrow().is_complete() && right.borrow().is_complete()
            }
            (None, Some(_)) => false,
            _ => true,
        }
    }

    pub fn traverse_pre_order(&self, result_recip: &mut String) {
        result_recip.push_str(format!("{} ", self.val).as_str());
        if let Some(left) = &self.left {
            left.borrow().traverse_pre_order(result_recip);
        }
        if let Some(right) = &self.right {
            right.borrow().traverse_pre_order(result_recip);
        }
    }

    pub fn traverse_in_order(&self, result_recip: &mut String) {
        if let Some(left) = &self.left {
            left.borrow().traverse_in_order(result_recip);
        }
        result_recip.push_str(format!("{} ", self.val).as_str());
        if let Some(right) = &self.right {
            right.borrow().traverse_in_order(result_recip);
        }
    }

    pub fn traverse_post_order(&self, result_recip: &mut String) {
        if let Some(left) = &self.left {
            left.borrow().traverse_post_order(result_recip);
        }
        if let Some(right) = &self.right {
            right.borrow().traverse_post_order(result_recip);
        }
        result_recip.push_str(format!("{} ", self.val).as_str());
    }
}

#[cfg(test)]
mod test {
    use std::sync::BarrierWaitResult;

    use super::*;

    #[test]
    fn linked_binary_tree_new() {
        let binary_tree = LinkedBinaryTree::<i32>::new();
        assert!(binary_tree.root.is_none());
    }

    #[test]
    fn linked_binary_tree_is_full() {
        let mut binary_tree = LinkedBinaryTree::<i32>::new();
        binary_tree.root = Some(Rc::new(RefCell::new(LinkedBinaryTreeNode::new(1))));
        assert!(binary_tree.is_full());
        binary_tree.root.as_mut().unwrap().borrow_mut().set_left(2);
        assert!(!binary_tree.is_full());
        binary_tree.root.as_mut().unwrap().borrow_mut().set_right(3);
        assert!(binary_tree.is_full());
        binary_tree
            .root
            .as_mut()
            .unwrap()
            .borrow_mut()
            .left
            .as_mut()
            .unwrap()
            .borrow_mut()
            .set_right(4);
        assert!(!binary_tree.is_full());
    }

    #[test]
    fn linked_binary_tree_is_complete() {
        let mut binary_tree = LinkedBinaryTree::<i32>::new();
        binary_tree.root = Some(Rc::new(RefCell::new(LinkedBinaryTreeNode::new(1))));
        binary_tree.root.as_mut().unwrap().borrow_mut().set_left(2);
        assert!(binary_tree.is_complete());
        binary_tree.root.as_mut().unwrap().borrow_mut().set_right(3);
        assert!(binary_tree.is_complete());
        binary_tree
            .root
            .as_mut()
            .unwrap()
            .borrow_mut()
            .left
            .as_mut()
            .unwrap()
            .borrow_mut()
            .set_right(4);
        assert!(!binary_tree.is_complete());
    }

    #[test]
    fn linked_binary_tree_trav_pre_order() {
        let mut binary_tree = LinkedBinaryTree::<i32>::new();
        binary_tree.root = Some(Rc::new(RefCell::new(LinkedBinaryTreeNode::new(1))));
        binary_tree.root.as_mut().unwrap().borrow_mut().set_left(2);
        binary_tree.root.as_mut().unwrap().borrow_mut().set_right(3);
        binary_tree
            .root
            .as_mut()
            .unwrap()
            .borrow_mut()
            .left
            .as_mut()
            .unwrap()
            .borrow_mut()
            .set_right(4);
        let mut result = String::new();
        binary_tree.traverse_pre_order(&mut result);
        assert_eq!("1 2 4 3 ", result.as_str());
    }

    #[test]
    fn linked_binary_tree_trav_in_order() {
        let mut binary_tree = LinkedBinaryTree::<i32>::new();
        binary_tree.root = Some(Rc::new(RefCell::new(LinkedBinaryTreeNode::new(1))));
        binary_tree.root.as_mut().unwrap().borrow_mut().set_left(2);
        binary_tree.root.as_mut().unwrap().borrow_mut().set_right(3);
        binary_tree
            .root
            .as_mut()
            .unwrap()
            .borrow_mut()
            .left
            .as_mut()
            .unwrap()
            .borrow_mut()
            .set_right(4);
        let mut result = String::new();
        binary_tree.traverse_in_order(&mut result);
        assert_eq!("2 4 1 3 ", result.as_str());
    }

    #[test]
    fn linked_binary_tree_trav_post_order() {
        let mut binary_tree = LinkedBinaryTree::<i32>::new();
        binary_tree.root = Some(Rc::new(RefCell::new(LinkedBinaryTreeNode::new(1))));
        binary_tree.root.as_mut().unwrap().borrow_mut().set_left(2);
        binary_tree.root.as_mut().unwrap().borrow_mut().set_right(3);
        binary_tree
            .root
            .as_mut()
            .unwrap()
            .borrow_mut()
            .left
            .as_mut()
            .unwrap()
            .borrow_mut()
            .set_right(4);
        let mut result = String::new();
        binary_tree.traverse_post_order(&mut result);
        assert_eq!("4 2 3 1 ", result.as_str());
    }

    #[test]
    fn linked_binary_tree_trav_level_order() {
        let mut binary_tree = LinkedBinaryTree::<i32>::new();
        binary_tree.root = Some(Rc::new(RefCell::new(LinkedBinaryTreeNode::new(1))));
        binary_tree.root.as_mut().unwrap().borrow_mut().set_left(2);
        binary_tree.root.as_mut().unwrap().borrow_mut().set_right(3);
        binary_tree
            .root
            .as_mut()
            .unwrap()
            .borrow_mut()
            .left
            .as_mut()
            .unwrap()
            .borrow_mut()
            .set_right(4);
        let mut result = String::new();
        binary_tree.traverse_level_order(&mut result);
        assert_eq!("1 2 3 4 ", result.as_str());
    }

    #[test]
    fn linked_binary_tree_count() {
        let mut binary_tree = LinkedBinaryTree::<i32>::new();
        binary_tree.root = Some(Rc::new(RefCell::new(LinkedBinaryTreeNode::new(1))));
        binary_tree.root.as_mut().unwrap().borrow_mut().set_left(2);
        binary_tree.root.as_mut().unwrap().borrow_mut().set_right(3);
        binary_tree
            .root
            .as_mut()
            .unwrap()
            .borrow_mut()
            .left
            .as_mut()
            .unwrap()
            .borrow_mut()
            .set_right(4);
        assert_eq!(4, binary_tree.count());
    }
}
