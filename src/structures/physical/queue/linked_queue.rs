use std::ops::Deref;

use super::{Queue, QueueErr};

pub struct LinkedQueue<T> {
    head: Option<Box<LinkedQueueNode<T>>>,
    len: usize,
    cap: usize,
}

impl<T> LinkedQueue<T> {
    pub fn with_capacity(cap: usize) -> Result<Self, QueueErr> {
        if cap == 0 {
            return Err(QueueErr::ZeroCapacityNotAllowed);
        }

        Ok(Self {
            head: None,
            len: 0,
            cap,
        })
    }

    pub fn iter(&self) -> LinkedQueueIter<T> {
        LinkedQueueIter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> LinkedQueueIterMut<T> {
        LinkedQueueIterMut {
            next: self.head.as_deref_mut(),
        }
    }

    fn get_tail_mut(&mut self) -> Option<&mut LinkedQueueNode<T>> {
        let mut linked_node_ref = &mut self.head;

        // Note on this tricky implementation :
        // The trick is NOT to borrow from anchor, and therefore to juggle
        // between two accumulators:
        // - one holding the reference to the current node
        // - the other being assigned the reference to the next node
        // Which leads to :
        //
        // let mut linked_node_ref = &mut self.head;
        // loop {
        //     // copy the reference to a tmp var
        //     let tmp = linked_node_ref;
        //     if let Some(ref mut node) = *tmp {
        //         linked_node_ref = &mut node.next;
        //     } else {
        //         linked_node_ref = tmp;
        //         break;
        //     }
        // }

        // improved by creating an unnamed temporary
        // The trick here is that using {anchor} moves the content of anchor
        // into an unnamed temporary on which the match executes.
        // Therefore, in the match block we are not borrowing from anchor but
        // from the temporary, leaving us free to modify anchor. See the
        // related blog post Stuff the Identity Function Does (in Rust).
        // Link : https://bluss.github.io/rust/fun/2015/10/11/stuff-the-identity-function-does/
        loop {
            match { linked_node_ref } {
                &mut Some(ref mut node) => {
                    if node.next.is_none() {
                        return Some(node);
                    }
                    linked_node_ref = &mut node.next;
                }
                n => return n.as_deref_mut(),
            }
        }
    }
}

struct LinkedQueueNode<T> {
    val: T,
    next: Option<Box<LinkedQueueNode<T>>>,
}

impl<T> Queue for LinkedQueue<T> {
    type Item = T;

    fn enqueue(&mut self, val: Self::Item) -> Result<(), QueueErr> {
        if self.is_full() {
            return Err(QueueErr::QueueOverflow);
        }

        if let Some(tail) = self.get_tail_mut() {
            tail.next = Some(Box::new(LinkedQueueNode { val, next: None }));
        } else {
            self.head = Some(Box::new(LinkedQueueNode { val, next: None }));
        }
        self.len += 1;
        Ok(())
    }

    fn dequeue(&mut self) -> Option<Self::Item> {
        self.head.take().map(|mut prev_head| {
            self.head = prev_head.next.take();
            prev_head.val
        })
    }

    fn peek_next(&self) -> Option<&Self::Item> {
        self.head.as_ref().map(|n| &n.val)
    }

    fn peek_next_mut(&mut self) -> Option<&mut Self::Item> {
        self.head.as_mut().map(|n| &mut n.val)
    }

    fn peek(&self, idx: usize) -> Option<&Self::Item> {
        for (i, val) in self.iter().enumerate() {
            if idx == i {
                return Some(val);
            }
        }
        None
    }

    fn peek_mut(&mut self, idx: usize) -> Option<&mut Self::Item> {
        for (i, val) in self.iter_mut().enumerate() {
            if idx == i {
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

pub struct LinkedQueueIter<'a, T> {
    next: Option<&'a LinkedQueueNode<T>>,
}

impl<'a, T> Iterator for LinkedQueueIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|n| {
            self.next = n.next.as_deref();
            &n.val
        })
    }
}

pub struct LinkedQueueIterMut<'a, T> {
    next: Option<&'a mut LinkedQueueNode<T>>,
}

impl<'a, T> Iterator for LinkedQueueIterMut<'a, T> {
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
    fn linked_queue_new() {
        let queue = LinkedQueue::<i32>::with_capacity(2);
        assert!(queue.is_ok());
        let queue = queue.unwrap();
        assert_eq!(0, queue.len);
        assert_eq!(2, queue.cap);
    }

    #[test]
    fn linked_queue_new_fail_zero_cap() {
        let queue = LinkedQueue::<i32>::with_capacity(0);
        assert!(queue.is_err());
        assert_eq!(Some(QueueErr::ZeroCapacityNotAllowed), queue.err());
    }

    #[test]
    fn linked_queue_enqueue() {
        let mut queue = LinkedQueue::<i32>::with_capacity(2).unwrap();
        let _ = queue.enqueue(1);
        assert!(queue.head.is_some());
        assert_eq!(1, queue.len);
        let _ = queue.enqueue(2);
        assert_eq!(2, queue.len);
    }

    #[test]
    fn linked_queue_iter() {
        let mut queue = LinkedQueue::<i32>::with_capacity(2).unwrap();
        let _ = queue.enqueue(1);
        let _ = queue.enqueue(2);
        let mut iter = queue.iter();
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn linked_queue_dequeue() {
        let mut queue = LinkedQueue::<i32>::with_capacity(2).unwrap();
        let _ = queue.enqueue(1);
        let _ = queue.enqueue(2);
        assert_eq!(Some(1), queue.dequeue());
        assert_eq!(Some(2), queue.dequeue());
        assert_eq!(None, queue.dequeue());
    }

    #[test]
    fn linked_queue_peek_next() {
        let mut queue = LinkedQueue::<i32>::with_capacity(2).unwrap();
        let _ = queue.enqueue(1);
        assert_eq!(Some(&1), queue.peek_next());
        let _ = queue.enqueue(2);
        assert_eq!(Some(&1), queue.peek_next());
    }

    #[test]
    fn linked_queue_peek_next_mut() {
        let mut queue = LinkedQueue::<i32>::with_capacity(2).unwrap();
        let _ = queue.enqueue(1);
        assert_eq!(Some(&mut 1), queue.peek_next_mut());
        let _ = queue.enqueue(2);
        assert_eq!(Some(&mut 1), queue.peek_next_mut());
    }

    #[test]
    fn linked_queue_peek() {
        let mut queue = LinkedQueue::<i32>::with_capacity(2).unwrap();
        let _ = queue.enqueue(1);
        let _ = queue.enqueue(2);
        assert_eq!(Some(&2), queue.peek(1));
        assert_eq!(Some(&1), queue.peek(0));
        assert_eq!(None, queue.peek(7));
    }

    #[test]
    fn linked_queue_peek_mut() {
        let mut queue = LinkedQueue::<i32>::with_capacity(2).unwrap();
        let _ = queue.enqueue(1);
        let _ = queue.enqueue(2);
        assert_eq!(Some(&mut 2), queue.peek_mut(1));
        assert_eq!(Some(&mut 1), queue.peek_mut(0));
        assert_eq!(None, queue.peek_mut(7));
    }
}
