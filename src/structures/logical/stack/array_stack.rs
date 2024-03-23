use std::vec;

use super::{Stack, StackErr};

#[derive(Debug)]
pub struct ArrayStack<T> {
    inner_stack: Box<[Option<T>]>,
    len: usize,
}

impl<T> ArrayStack<T> {
    pub fn with_capacity(capacity: usize) -> Result<Self, StackErr> {
        if capacity == 0 {
            return Err(StackErr::ZeroCapacityNotAllowed);
        };

        // Option doesn't implement clone so vec![None; capacity] can't be used
        let mut inner_stack = Vec::with_capacity(capacity);
        (0..capacity).for_each(|_| {
            inner_stack.push(None);
        });

        Ok(Self {
            inner_stack: inner_stack.into_boxed_slice(),
            len: 0,
        })
    }

    pub fn cap(&self) -> usize {
        self.inner_stack.len()
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Stack for ArrayStack<T> {
    type Item = T;

    fn push(&mut self, val: Self::Item) -> Result<(), StackErr> {
        if self.is_full() {
            return Err(StackErr::StackOverflow);
        }
        self.inner_stack[self.len] = Some(val);
        self.len += 1;
        Ok(())
    }

    fn pop(&mut self) -> Option<Self::Item> {
        let item = self.inner_stack[self.len].take();
        self.len -= 1;
        item
    }

    fn peek_next(&self) -> Option<&Self::Item> {
        self.inner_stack
            .get(self.len().saturating_sub(1))
            .and_then(|item| item.as_ref())
    }

    fn peek_next_mut(&mut self) -> Option<&mut Self::Item> {
        self.inner_stack
            .get_mut(self.len().saturating_sub(1))
            .and_then(|item| item.as_mut())
    }

    fn peek(&self, idx: usize) -> Option<&Self::Item> {
        self.inner_stack.get(idx).and_then(|item| item.as_ref())
    }

    fn peek_mut(&mut self, idx: usize) -> Option<&mut Self::Item> {
        self.inner_stack.get_mut(idx).and_then(|item| item.as_mut())
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len == self.inner_stack.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn array_stack_new() {
        let stack_new_res = ArrayStack::<i32>::with_capacity(5);
        assert!(stack_new_res.is_ok());
    }

    #[test]
    fn array_stack_new_fail_with_zero() {
        let stack_new_res = ArrayStack::<i32>::with_capacity(0);
        assert!(stack_new_res.is_err());
        assert_eq!(
            StackErr::ZeroCapacityNotAllowed,
            stack_new_res.err().unwrap()
        );
    }

    #[test]
    fn array_stack_capacity() {
        let stack_new_res = ArrayStack::<i32>::with_capacity(5).unwrap();
        assert_eq!(5, stack_new_res.cap());
    }

    #[test]
    fn array_stack_len() {
        let stack_new_res = ArrayStack::<i32>::with_capacity(5).unwrap();
        assert_eq!(0, stack_new_res.len());
    }

    #[test]
    fn array_stack_is_empty() {
        let stack_new_res = ArrayStack::<i32>::with_capacity(5).unwrap();
        assert!(stack_new_res.is_empty());
    }

    #[test]
    fn array_stack_is_full() {
        let mut stack_new_res = ArrayStack::<i32>::with_capacity(2).unwrap();
        stack_new_res.push(1).unwrap();
        stack_new_res.push(2).unwrap();
        assert!(stack_new_res.is_full());
    }

    #[test]
    fn array_stack_push() {
        let mut stack_new_res = ArrayStack::<i32>::with_capacity(5).unwrap();
        stack_new_res.push(1).unwrap();
        stack_new_res.push(2).unwrap();
        assert_eq!(2, stack_new_res.len());
    }

    #[test]
    fn array_stack_pop() {
        let mut stack_new_res = ArrayStack::<i32>::with_capacity(5).unwrap();
        stack_new_res.push(1).unwrap();
        stack_new_res.push(2).unwrap();
        stack_new_res.pop();
        assert_eq!(1, stack_new_res.len());
    }

    #[test]
    fn array_stack_peek_next() {
        let mut stack_new_res = ArrayStack::<i32>::with_capacity(5).unwrap();
        assert_eq!(None, stack_new_res.peek_next());
        stack_new_res.push(1).unwrap();
        stack_new_res.push(2).unwrap();
        assert_eq!(Some(&2), stack_new_res.peek_next());
        stack_new_res.push(3).unwrap();
        assert_eq!(Some(&3), stack_new_res.peek_next());
    }

    #[test]
    fn array_stack_peek_next_mut() {
        let mut stack_new_res = ArrayStack::<i32>::with_capacity(5).unwrap();
        stack_new_res.push(1).unwrap();
        stack_new_res.push(2).unwrap();
        assert_eq!(Some(&2), stack_new_res.peek_next());
        stack_new_res.peek_next_mut().map(|v| *v = 10).unwrap();
        assert_eq!(Some(&10), stack_new_res.peek_next());
    }

    #[test]
    fn array_stack_peek() {
        let mut stack_new_res = ArrayStack::<i32>::with_capacity(5).unwrap();
        assert_eq!(None, stack_new_res.peek(0));
        stack_new_res.push(1).unwrap();
        stack_new_res.push(2).unwrap();
        assert_eq!(Some(&1), stack_new_res.peek(0));
        assert_eq!(Some(&2), stack_new_res.peek(1));
    }

    #[test]
    fn array_stack_peek_mut() {
        let mut stack_new_res = ArrayStack::<i32>::with_capacity(5).unwrap();
        assert_eq!(None, stack_new_res.peek(0));
        stack_new_res.push(1).unwrap();
        stack_new_res.push(2).unwrap();
        assert_eq!(Some(&1), stack_new_res.peek(0));
        assert_eq!(Some(&2), stack_new_res.peek(1));
        stack_new_res.peek_mut(0).map(|v| *v = 7).unwrap();
        assert_eq!(Some(&7), stack_new_res.peek(0));
    }
}
