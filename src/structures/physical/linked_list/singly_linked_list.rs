#![allow(dead_code)]

use super::LinkedList;

#[derive(Debug)]
struct SinglyLinkedNode<T> {
    val: T,
    next: Option<Box<SinglyLinkedNode<T>>>,
}

impl<T> SinglyLinkedNode<T> {
    fn new(val: T, next: Option<Box<SinglyLinkedNode<T>>>) -> Self {
        Self { val, next }
    }
}

pub struct SinglyLinkedList<T> {
    head: Option<Box<SinglyLinkedNode<T>>>,
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn iter(&self) -> SinglyLinkedIter<T> {
        SinglyLinkedIter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> SinglyLinkedIterMut<T> {
        SinglyLinkedIterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> LinkedList<T> for SinglyLinkedList<T> {
    fn push(&mut self, val: T) {
        if self.head.is_none() {
            self.head = Some(Box::new(SinglyLinkedNode::new(val, None)))
        } else {
            let prev_head = self.head.take();
            self.head = Some(Box::new(SinglyLinkedNode::new(val, prev_head)))
        }
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|mut prev_head| {
            self.head = prev_head.next.take();
            prev_head.val
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.val)
    }

    fn reverse(&mut self) {
        todo!()
    }
}

pub struct SinglyLinkedIter<'a, T> {
    next: Option<&'a SinglyLinkedNode<T>>,
}

impl<'a, T> Iterator for SinglyLinkedIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|n| {
            self.next = n.next.as_deref();
            &n.val
        })
    }
}

pub struct SinglyLinkedIterMut<'a, T> {
    next: Option<&'a mut SinglyLinkedNode<T>>,
}

impl<'a, T> Iterator for SinglyLinkedIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|n| {
            self.next = n.next.as_deref_mut();
            &mut n.val
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn singly_linked_list_new() {
        let list = SinglyLinkedList::<i32>::new();
        assert!(list.head.is_none());
    }

    #[test]
    fn singly_linked_list_push() {
        let mut list = SinglyLinkedList::<i32>::new();
        list.push(1);
        assert!(list.head.is_some());
        list.push(2);
        assert!(list.head.is_some());
    }

    #[test]
    fn singly_linked_list_pop() {
        let mut list = SinglyLinkedList::<i32>::new();
        list.push(1);
        list.push(2);
        assert_eq!(Some(2), list.pop());
        assert!(list.head.is_some());
        assert_eq!(Some(1), list.pop());
        assert!(list.head.is_none());
        assert_eq!(None, list.pop());
    }

    #[test]
    fn singly_linked_list_peek() {
        let mut list = SinglyLinkedList::<i32>::new();
        assert_eq!(None, list.peek());
        list.push(1);
        list.push(2);
        assert_eq!(Some(&2), list.peek());
        list.pop();
        list.pop();
        assert_eq!(None, list.peek());
    }

    #[test]
    fn singly_linked_list_peek_mut() {
        let mut list = SinglyLinkedList::<i32>::new();
        assert_eq!(None, list.peek());
        list.push(1);
        list.push(2);
        if let Some(v) = list.peek_mut() {
            *v = 7;
        }
        assert_eq!(Some(&7), list.peek());
    }

    #[test]
    fn singly_linked_list_iter() {
        let mut list = SinglyLinkedList::<i32>::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.iter();
        assert_eq!(Some(&3), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn singly_linked_list_iter_mut() {
        let mut list = SinglyLinkedList::<i32>::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.iter_mut();
        assert_eq!(Some(&mut 3), iter.next());
        assert_eq!(Some(&mut 2), iter.next());
        assert_eq!(Some(&mut 1), iter.next());
        assert_eq!(None, iter.next());
    }
}
