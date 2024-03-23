#![allow(dead_code)]

use std::{
    cell::{Cell, RefCell},
    ops::BitXorAssign,
    rc::{Rc, Weak},
};

pub trait BinaryTree {
    type T;

    fn is_full(&self) -> bool;
    fn is_perfect(&self) -> bool;
    fn is_complete(&self) -> bool;
    fn is_balanced(&self) -> bool;

    // DFS
    fn traverse_pre_order(&self);
    fn traverse_in_order(&self);
    fn traverse_post_order(&self);

    // BFS
    fn traverse_level_order(&self);
}

// TODO: Make a binary tree based on an array ?

type LinkedBinaryTreeNodeRef<T> = Rc<RefCell<LinkedBinaryTreeNode<T>>>;

pub struct LinkedBinaryTree<T> {
    root: Option<LinkedBinaryTreeNodeRef<T>>,
}

impl<T> LinkedBinaryTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }
}

struct LinkedBinaryTreeNode<T> {
    val: T,
    left: Option<LinkedBinaryTreeNodeRef<T>>,
    right: Option<LinkedBinaryTreeNodeRef<T>>,
}

impl<T> LinkedBinaryTreeNode<T> {
    pub fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn set_left(&mut self, val: T) {
        self.left = Some(Rc::new(RefCell::new(LinkedBinaryTreeNode::new(val))));
    }

    pub fn set_right(&mut self, val: T) {
        self.right = Some(Rc::new(RefCell::new(LinkedBinaryTreeNode::new(val))));
    }

    fn is_full(&self) -> bool {
        match (&self.left, &self.right) {
            (Some(left), Some(right)) => left.borrow().is_full() && right.borrow().is_full(),
            (None, None) => true,
            _ => false,
        }
    }

    fn is_complete(&self) -> bool {
        match (&self.left, &self.right) {
            (Some(left), Some(right)) => {
                left.borrow().is_complete() && right.borrow().is_complete()
            }
            (None, Some(_)) => false,
            _ => true,
        }
    }
}

impl<T> BinaryTree for LinkedBinaryTree<T> {
    type T = T;

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

    fn traverse_pre_order(&self) {
        todo!()
    }

    fn traverse_in_order(&self) {
        todo!()
    }

    fn traverse_post_order(&self) {
        todo!()
    }

    fn traverse_level_order(&self) {
        todo!()
    }
}

#[cfg(test)]
mod test {
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
        assert!(binary_tree.is_complete());
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
}
