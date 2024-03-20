use super::{Stack, StackErr};

#[derive(Debug)]
struct LinkedStackNode<T> {
    val: T,
    next: Option<Box<LinkedStackNode<T>>>,
}

impl<T> LinkedStackNode<T> {
    fn new(val: T, next: Option<Box<LinkedStackNode<T>>>) -> Self {
        Self { val, next }
    }
}

#[derive(Debug)]
pub struct LinkedStack<T> {
    top: Option<Box<LinkedStackNode<T>>>,
    cap: usize,
    len: usize,
}

impl<T> LinkedStack<T> {
    pub fn with_capacity(cap: usize) -> Result<Self, StackErr> {
        if cap == 0 {
            return Err(StackErr::ZeroCapacityNotAllowed);
        }

        Ok(Self {
            top: None,
            cap,
            len: 0,
        })
    }

    pub fn cap(&self) -> usize {
        self.cap
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> LinkedStackIter<T> {
        return LinkedStackIter {
            next: self.top.as_deref(),
        };
    }

    pub fn iter_mut(&mut self) -> LinkedStackIterMut<T> {
        return LinkedStackIterMut {
            next: self.top.as_deref_mut(),
        };
    }
}

impl<T> Stack for LinkedStack<T> {
    type Item = T;

    fn push(&mut self, val: Self::Item) -> Result<(), StackErr> {
        if self.is_full() {
            return Err(StackErr::StackOverflow);
        }
        let new_top = Box::new(LinkedStackNode::new(val, self.top.take()));
        self.top = Some(new_top);
        self.len += 1;
        Ok(())
    }

    fn pop(&mut self) -> Option<Self::Item> {
        match self.top.take() {
            Some(mut old_top) => {
                self.top = old_top.next.take();
                self.len -= 1;
                Some(old_top.val)
            }
            None => None,
        }
    }

    fn peek_next(&self) -> Option<&Self::Item> {
        self.top.as_ref().map(|node| &node.val)
    }

    fn peek_next_mut(&mut self) -> Option<&mut Self::Item> {
        self.top.as_mut().map(|node| &mut node.val)
    }

    fn peek(&self, idx: usize) -> Option<&Self::Item> {
        if idx > self.len || self.is_empty() {
            return None;
        }

        if idx == 0 {
            // Safe to unwrap the stack is not empty so first index is not None
            return self.top.as_ref().map(|node| &node.val);
        }

        for (i, val) in self.iter().enumerate() {
            if i == idx {
                return Some(val);
            }
        }

        None
    }

    fn peek_mut(&mut self, idx: usize) -> Option<&mut Self::Item> {
        if idx > self.len || self.is_empty() {
            return None;
        }

        if idx == 0 {
            // Safe to unwrap the stack is not empty so first index is not None
            return self.top.as_mut().map(|node| &mut node.val);
        }

        for (i, val) in self.iter_mut().enumerate() {
            if i == idx {
                return Some(val);
            }
        }

        None
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len == self.cap
    }
}

pub struct LinkedStackIter<'a, T> {
    next: Option<&'a LinkedStackNode<T>>,
}

impl<'a, T> Iterator for LinkedStackIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|curr_node| {
            self.next = curr_node.next.as_deref();
            &curr_node.val
        })
    }
}

pub struct LinkedStackIterMut<'a, T> {
    next: Option<&'a mut LinkedStackNode<T>>,
}

impl<'a, T> Iterator for LinkedStackIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|curr_node| {
            self.next = curr_node.next.as_deref_mut();
            &mut curr_node.val
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn linked_stack_new() {
        let linked_stack = LinkedStack::<i32>::with_capacity(5);
        assert!(linked_stack.is_ok());
    }

    #[test]
    fn linked_stack_new_with_zero_cap() {
        let linked_stack = LinkedStack::<i32>::with_capacity(0);
        assert!(linked_stack.is_err());
    }

    #[test]
    fn linked_stack_cap() {
        let linked_stack = LinkedStack::<i32>::with_capacity(5).unwrap();
        assert_eq!(5, linked_stack.cap());
    }

    #[test]
    fn linked_stack_len() {
        let linked_stack = LinkedStack::<i32>::with_capacity(5).unwrap();
        assert_eq!(0, linked_stack.len());
    }

    #[test]
    fn linked_stack_push() {
        let mut linked_stack = LinkedStack::<i32>::with_capacity(2).unwrap();
        linked_stack.push(1).unwrap();
        linked_stack.push(2).unwrap();
        assert_eq!(2, linked_stack.len());
        let push_res = linked_stack.push(3);
        assert!(push_res.is_err());
        assert_eq!(StackErr::StackOverflow, push_res.err().unwrap());
    }

    #[test]
    fn linked_stack_pop() {
        let mut linked_stack = LinkedStack::<i32>::with_capacity(2).unwrap();
        linked_stack.push(1).unwrap();
        linked_stack.push(2).unwrap();
        assert_eq!(2, linked_stack.len());
        assert_eq!(Some(2), linked_stack.pop());
        assert_eq!(1, linked_stack.len());
    }

    #[test]
    fn linked_stack_iter() {
        let mut linked_stack = LinkedStack::<i32>::with_capacity(3).unwrap();
        linked_stack.push(1).unwrap();
        linked_stack.push(2).unwrap();
        linked_stack.push(3).unwrap();
        let mut iter = linked_stack.iter();
        assert_eq!(Some(&3), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn linked_stack_iter_mut() {
        let mut linked_stack = LinkedStack::<i32>::with_capacity(3).unwrap();
        linked_stack.push(1).unwrap();
        linked_stack.push(2).unwrap();
        linked_stack.push(3).unwrap();
        let mut iter = linked_stack.iter_mut();
        assert_eq!(Some(&mut 3), iter.next());
        assert_eq!(Some(&mut 2), iter.next());
        assert_eq!(Some(&mut 1), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn linked_stack_peek_next() {
        let mut linked_stack = LinkedStack::<i32>::with_capacity(3).unwrap();
        linked_stack.push(1).unwrap();
        linked_stack.push(2).unwrap();
        linked_stack.push(3).unwrap();
        assert_eq!(Some(&3), linked_stack.peek_next());
        linked_stack.pop();
        assert_eq!(Some(&2), linked_stack.peek_next());
        linked_stack.pop();
        linked_stack.pop();
        assert_eq!(None, linked_stack.peek_next());
    }

    #[test]
    fn linked_stack_peek() {
        let mut linked_stack = LinkedStack::<i32>::with_capacity(3).unwrap();
        linked_stack.push(1).unwrap();
        linked_stack.push(2).unwrap();
        linked_stack.push(3).unwrap();
        assert_eq!(Some(&2), linked_stack.peek(1));
        assert_eq!(Some(&1), linked_stack.peek(2));
        assert_eq!(Some(&3), linked_stack.peek(0));
        assert_eq!(None, linked_stack.peek(5));
    }

    #[test]
    fn linked_stack_peek_mut() {
        let mut linked_stack = LinkedStack::<i32>::with_capacity(3).unwrap();
        linked_stack.push(1).unwrap();
        linked_stack.push(2).unwrap();
        linked_stack.push(3).unwrap();
        assert_eq!(Some(&mut 2), linked_stack.peek_mut(1));
        assert_eq!(Some(&mut 1), linked_stack.peek_mut(2));
        assert_eq!(Some(&mut 3), linked_stack.peek_mut(0));
        assert_eq!(None, linked_stack.peek_mut(5));
    }
}
