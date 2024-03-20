use std::fmt::Display;

pub use singly_linked_list::SinglyLinkedList;

mod singly_linked_list;

/// Linked list abstract data type. Contains all the expected behaviours that
/// a linked list should give.
pub trait LinkedList<T> {
    /// Insert a new value in the linked list. This method is expected to have
    /// a time complexity of O(n).
    fn push(&mut self, val: T);

    /// Pop the head of the linked list, the time complexity is expected to be
    /// O(1)
    fn pop(&mut self) -> Option<T>;

    /// Return a reference to the head value. Expected time complexity is O(1).
    fn peek(&self) -> Option<&T>;

    /// Return a mutable reference to the head value. Expected time complexity
    /// is O(1).
    fn peek_mut(&mut self) -> Option<&mut T>;

    /// Reverse the order of the list. Expected time complexity is O(n).
    fn reverse(&mut self);
}
