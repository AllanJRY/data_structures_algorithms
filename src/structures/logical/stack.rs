#![allow(dead_code)]

use std::{
    cell::{Ref, RefCell, RefMut},
    collections::VecDeque,
    rc::Rc,
};

// TODO: add documentation same as queue but note that Stack is LIFO
pub struct Stack<T> {
    content: VecDeque<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            content: VecDeque::new(),
        }
    }

    pub fn push(&mut self, el: T) {
        self.content.push_back(el);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.content.pop_back()
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.content.back()
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.content.back_mut()
    }
}

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self { val, next: None }
    }
}

#[derive(Debug)]
struct NodeStack<T> {
    len: usize,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> NodeStack<T> {
    fn new() -> Self {
        Self {
            len: 0,
            head: None,
            tail: None,
        }
    }

    pub fn push(&mut self, val: T) {
        let node = Rc::new(RefCell::new(Node::new(val)));
        match self.len {
            0 => {
                self.head = Some(node.clone());
                self.tail = Some(node);
            }
            1 => {
                node.borrow_mut().next = self.head.clone();
                self.tail = Some(node);
            }
            _ => {
                if let Some(prev_tail) = self.tail.take() {
                    node.borrow_mut().next = Some(prev_tail);
                    self.tail = Some(node);
                }
            }
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(prev_tail) = self.tail.take() {
            // if len = 1, head and tail references the same value, so the Rc
            // on prev_tail as a ref count of two we need to empty the head
            // to drop his ref so the Rc decrease to 1 ref, and the Rc::try_unwrap
            // will not fail.
            if self.len == 1 {
                self.head = None;
            }
            self.tail = prev_tail.borrow().next.clone();
            self.len -= 1;
            if let Ok(node_ref_cell) = Rc::try_unwrap(prev_tail) {
                Some(node_ref_cell.into_inner().val)
            } else {
                panic!("Error during unwraping of Rc, other references may still exist.");
            }
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|tail_ref| Ref::map(tail_ref.borrow(), |tail_node_ref| &tail_node_ref.val))
    }

    pub fn peek_mut(&self) -> Option<RefMut<T>> {
        self.tail.as_ref().map(|tail_ref| {
            RefMut::map(tail_ref.borrow_mut(), |tail_node_ref| {
                &mut tail_node_ref.val
            })
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stack_push_peek() {
        let mut stack = Stack::<u32>::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.peek(), Some(&2));
    }

    #[test]
    fn stack_pop() {
        let mut stack = Stack::<u32>::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2));
    }

    #[test]
    fn stack_peek_mut() {
        let mut stack = Stack::<u32>::new();
        stack.push(1);
        if let Some(el) = stack.peek_mut() {
            *el = 77;
        }
        assert_eq!(stack.peek(), Some(&77));
    }

    #[test]
    fn node_stack_push_peek() {
        let mut node_stack = NodeStack::<u32>::new();
        node_stack.push(1);
        node_stack.push(2);
        node_stack.push(3);
        assert_eq!(&*node_stack.peek().unwrap(), &3);
    }

    #[test]
    fn node_stack_pop() {
        let mut node_stack: NodeStack<u32> = NodeStack::new();
        node_stack.push(3);
        node_stack.push(7);
        node_stack.push(5);
        node_stack.push(1);
        assert_eq!(node_stack.pop(), Some(1));
        assert_eq!(node_stack.pop(), Some(5));
        assert_eq!(node_stack.pop(), Some(7));
        assert_eq!(node_stack.pop(), Some(3));
    }

    #[test]
    fn node_stack_peek_mut() {
        let mut node_stack: NodeStack<u32> = NodeStack::new();
        node_stack.push(3);
        node_stack.push(7);
        if let Some(mut val) = node_stack.peek_mut() {
            *val = 77;
        }
        assert_eq!(&*node_stack.peek().unwrap(), &77);
    }
}
