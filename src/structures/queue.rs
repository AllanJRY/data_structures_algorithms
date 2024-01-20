#![allow(dead_code)]
use std::{
    cell::{Ref, RefCell, RefMut},
    collections::VecDeque,
    rc::Rc,
};

// TODO: add documentations + add note about the fact that a linkedlist queue is hard
// to implement in rust, and this one is a contrained interface over VecDeque to
// use it as a simple queue + (Queue = FIFO)
pub struct Queue<T> {
    content: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            content: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, el: T) {
        self.content.push_back(el)
    }

    pub fn deque(&mut self) -> Option<T> {
        self.content.pop_front()
    }

    pub fn peek(&self) -> Option<&T> {
        self.content.front()
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.content.front_mut()
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
struct NodeQueue<T> {
    len: usize,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> NodeQueue<T> {
    fn new() -> Self {
        Self {
            len: 0,
            head: None,
            tail: None,
        }
    }

    pub fn enqueue(&mut self, val: T) {
        let node = Rc::new(RefCell::new(Node::new(val)));
        match self.len {
            0 => {
                self.head = Some(node.clone());
                self.tail = Some(node);
            }
            1 => {
                if let Some(head) = &self.head {
                    head.borrow_mut().next = Some(node.clone());
                }
                self.tail = Some(node);
            }
            _ => {
                if let Some(prev_tail) = self.tail.take() {
                    prev_tail.borrow_mut().next = Some(node.clone());
                    self.tail = Some(prev_tail);
                }
            }
        }
        self.len += 1;
    }

    pub fn deque(&mut self) -> Option<T> {
        if let Some(prev_head) = self.head.take() {
            // if len = 1, head and tail references the same value, so the Rc
            // on prev_head as a ref count of two we need to empty the tail
            // to drop his ref so the Rc decrease to 1 ref, and the Rc::try_unwrap
            // will not fail.
            if self.len == 1 {
                self.tail = None;
            }
            self.head = prev_head.borrow().next.clone();
            self.len -= 1;
            if let Ok(node_ref_cell) = Rc::try_unwrap(prev_head) {
                Some(node_ref_cell.into_inner().val)
            } else {
                panic!("Error during unwraping of Rc, other references may still exist.");
            }
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|head_ref| Ref::map(head_ref.borrow(), |head_node_ref| &head_node_ref.val))
    }

    pub fn peek_mut(&self) -> Option<RefMut<T>> {
        self.head.as_ref().map(|head_ref| {
            RefMut::map(head_ref.borrow_mut(), |head_node_ref| {
                &mut head_node_ref.val
            })
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn queue_enqueue_peek() {
        let mut queue: Queue<u32> = Queue::new();
        queue.enqueue(3);
        queue.enqueue(7);
        assert_eq!(queue.peek(), Some(&3));
    }

    #[test]
    fn queue_deque() {
        let mut queue: Queue<u32> = Queue::new();
        queue.enqueue(3);
        queue.enqueue(7);
        assert_eq!(queue.deque(), Some(3));
    }

    #[test]
    fn queue_peek_mut() {
        let mut queue: Queue<u32> = Queue::new();
        queue.enqueue(3);
        if let Some(el) = queue.peek_mut() {
            *el = 77;
        }
        assert_eq!(queue.peek(), Some(&77));
    }

    #[test]
    fn node_queue_enqueue() {
        let mut node_queue: NodeQueue<u32> = NodeQueue::new();
        node_queue.enqueue(3);
        node_queue.enqueue(7);
        assert_eq!(&*node_queue.peek().unwrap(), &3);
    }

    #[test]
    fn node_queue_deque() {
        let mut node_queue: NodeQueue<u32> = NodeQueue::new();
        node_queue.enqueue(3);
        node_queue.enqueue(7);
        assert_eq!(node_queue.deque(), Some(3));
        assert_eq!(&*node_queue.peek().unwrap(), &7);
        assert_eq!(node_queue.deque(), Some(7));
    }

    #[test]
    fn node_queue_peek_mut() {
        let mut node_queue: NodeQueue<u32> = NodeQueue::new();
        node_queue.enqueue(3);
        node_queue.enqueue(7);
        if let Some(mut el) = node_queue.peek_mut() {
            *el = 77;
        }
        assert_eq!(&*node_queue.peek().unwrap(), &77);
    }
}
