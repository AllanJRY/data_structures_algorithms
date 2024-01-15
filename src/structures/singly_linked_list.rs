#![allow(dead_code)]

// TODO: add documentations
#[derive(Debug)]
pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SinglyLinkedList<T> {
    pub fn empty() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, val: T) {
        // the methode take is an "alias" for std::mem::replace(&mut self.head, None)
        let prev_head = self.head.take();
        let new_head = Node {
            val,
            next: prev_head,
        };
        self.head = Some(Box::new(new_head));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|poped_val| {
            self.head = poped_val.next;
            poped_val.val
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.val)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_linked_list() {
        let list: SinglyLinkedList<i32> = SinglyLinkedList::empty();
        assert_eq!(None, list.head);
    }

    #[test]
    fn linked_list_push_single() {
        let mut list: SinglyLinkedList<i32> = SinglyLinkedList::empty();
        list.push(3);
        assert!(list.head.is_some());
        assert!(list
            .head
            .is_some_and(|node| node.val == 3 && node.next.is_none()));
    }

    #[test]
    fn linked_list_push_mutilple() {
        let mut list: SinglyLinkedList<i32> = SinglyLinkedList::empty();
        list.push(3);
        list.push(7);
        list.push(2);
        assert!(list.head.is_some());
        assert!(list.head.is_some_and(|node| node.val == 2
            && node.next.is_some_and(|next_node| next_node.val == 7
                && next_node
                    .next
                    .is_some_and(|tail| tail.val == 3 && tail.next.is_none()))));
    }

    #[test]
    fn linked_list_pop_single() {
        let mut list: SinglyLinkedList<i32> = SinglyLinkedList::empty();
        list.push(3);
        assert!(list.head.is_some());
        let val = list.pop();
        assert_eq!(Some(3), val);
        assert!(list.head.is_none());
    }

    #[test]
    fn linked_list_pop_multiple() {
        let mut list: SinglyLinkedList<i32> = SinglyLinkedList::empty();
        list.push(3);
        list.push(7);
        list.push(2);
        let val = list.pop();
        assert_eq!(Some(2), val);
        let val = list.pop();
        assert_eq!(Some(7), val);
        let val = list.pop();
        assert_eq!(Some(3), val);
        assert!(list.head.is_none());
    }

    #[test]
    fn linked_list_peek() {
        let mut list: SinglyLinkedList<i32> = SinglyLinkedList::empty();
        list.push(3);
        assert_eq!(Some(&3), list.peek());
    }
}
