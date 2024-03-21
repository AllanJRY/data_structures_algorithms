use std::ops::{Deref, DerefMut};

use super::{Queue, QueueErr};

pub struct ArrayQueue<T> {
    inner_queue: Box<[Option<T>]>,
    len: usize,
    cap: usize,
}

impl<T> ArrayQueue<T> {
    pub fn with_capacity(cap: usize) -> Result<Self, QueueErr> {
        if cap == 0 {
            return Err(QueueErr::ZeroCapacityNotAllowed);
        }

        let mut inner_queue = Vec::new();
        (0..cap).for_each(|_| {
            inner_queue.push(None);
        });

        Ok(Self {
            inner_queue: inner_queue.into_boxed_slice(),
            len: 0,
            cap,
        })
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Option<T>> {
        self.inner_queue.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Option<T>> {
        self.inner_queue.iter_mut()
    }
}

impl<T> Queue for ArrayQueue<T> {
    type Item = T;

    fn enqueue(&mut self, val: Self::Item) -> Result<(), QueueErr> {
        if self.is_full() {
            Err(QueueErr::QueueOverflow)
        } else {
            self.inner_queue[self.len] = Some(val);
            self.len += 1;
            Ok(())
        }
    }

    fn dequeue(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            None
        } else {
            let prev_head = self.inner_queue[0].take();
            self.len -= 1;
            // downside here: shifting happen even if all element are None
            // in case of a queue with one element left
            for i in 0..self.len {
                self.inner_queue.swap(i, i + 1);
            }
            prev_head
        }
    }

    fn peek_next(&self) -> Option<&Self::Item> {
        self.inner_queue[0].as_ref()
    }

    fn peek_next_mut(&mut self) -> Option<&mut Self::Item> {
        self.inner_queue[0].as_mut()
    }

    fn peek(&self, idx: usize) -> Option<&Self::Item> {
        self.inner_queue[idx].as_ref()
    }

    fn peek_mut(&mut self, idx: usize) -> Option<&mut Self::Item> {
        self.inner_queue[idx].as_mut()
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len == self.cap
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn array_queue_new() {
        let queue = ArrayQueue::<i32>::with_capacity(5);
        assert!(queue.is_ok());
        let queue = queue.unwrap();
        assert_eq!(5, queue.cap);
        assert_eq!(0, queue.len);
    }

    #[test]
    fn array_queue_new_zero_cap() {
        let queue = ArrayQueue::<i32>::with_capacity(0);
        assert!(queue.is_err());
        assert_eq!(QueueErr::ZeroCapacityNotAllowed, queue.err().unwrap())
    }

    #[test]
    fn array_queue_is_empty() {
        let queue = ArrayQueue::<i32>::with_capacity(2).unwrap();
        assert!(queue.is_empty());
    }

    #[test]
    fn array_queue_enqueue() {
        let mut queue = ArrayQueue::<i32>::with_capacity(2).unwrap();
        let _ = queue.enqueue(1);
        let _ = queue.enqueue(2);
        assert_eq!(Some(1), queue.inner_queue[0]);
        assert_eq!(Some(2), queue.inner_queue[1]);
    }

    #[test]
    fn array_queue_enqueue_overflow() {
        let mut queue = ArrayQueue::<i32>::with_capacity(2).unwrap();
        let _ = queue.enqueue(1);
        let _ = queue.enqueue(2);
        let res = queue.enqueue(3);
        assert!(res.is_err());
        assert_eq!(Some(QueueErr::QueueOverflow), res.err());
    }

    #[test]
    fn array_queue_is_full() {
        let mut queue = ArrayQueue::<i32>::with_capacity(2).unwrap();
        let _ = queue.enqueue(1);
        let _ = queue.enqueue(2);
        assert!(queue.is_full());
        assert!(!queue.is_empty());
    }

    #[test]
    fn array_queue_dequeue() {
        let mut queue = ArrayQueue::<i32>::with_capacity(2).unwrap();
        let _ = queue.enqueue(1);
        let _ = queue.enqueue(2);
        assert_eq!(Some(1), queue.dequeue());
        assert_eq!(Some(2), queue.dequeue());
        assert_eq!(None, queue.dequeue());
    }

    #[test]
    fn array_queue_peek_next() {
        let mut queue = ArrayQueue::<i32>::with_capacity(2).unwrap();
        let _ = queue.enqueue(1);
        let _ = queue.enqueue(2);
        assert_eq!(Some(&1), queue.peek_next());
    }

    #[test]
    fn array_queue_peek_next_mut() {
        let mut queue = ArrayQueue::<i32>::with_capacity(2).unwrap();
        let _ = queue.enqueue(1);
        let _ = queue.enqueue(2);
        assert_eq!(Some(&mut 1), queue.peek_next_mut());
    }

    #[test]
    fn array_queue_peek() {
        let mut queue = ArrayQueue::<i32>::with_capacity(2).unwrap();
        let _ = queue.enqueue(1);
        let _ = queue.enqueue(2);
        assert_eq!(Some(&1), queue.peek(0));
        assert_eq!(Some(&2), queue.peek(1));
    }

    #[test]
    fn array_queue_peek_mut() {
        let mut queue = ArrayQueue::<i32>::with_capacity(2).unwrap();
        let _ = queue.enqueue(1);
        let _ = queue.enqueue(2);
        assert_eq!(Some(&mut 1), queue.peek_mut(0));
        assert_eq!(Some(&mut 2), queue.peek_mut(1));
    }

    #[test]
    fn array_queue_iter() {
        let mut queue = ArrayQueue::<i32>::with_capacity(3).unwrap();
        let _ = queue.enqueue(1);
        let _ = queue.enqueue(2);
        let _ = queue.enqueue(3);
        let mut iter = queue.iter();
        assert_eq!(Some(&Some(1)), iter.next());
        assert_eq!(Some(&Some(2)), iter.next());
        assert_eq!(Some(&Some(3)), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn array_queue_iter_mut() {
        let mut queue = ArrayQueue::<i32>::with_capacity(3).unwrap();
        let _ = queue.enqueue(1);
        let _ = queue.enqueue(2);
        let _ = queue.enqueue(3);
        let mut iter = queue.iter_mut();
        assert_eq!(Some(&mut Some(1)), iter.next());
        assert_eq!(Some(&mut Some(2)), iter.next());
        assert_eq!(Some(&mut Some(3)), iter.next());
        assert_eq!(None, iter.next());
    }
}
