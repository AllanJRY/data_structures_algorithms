#![allow(dead_code)]
// TODO : implementations :
//  - ArrayQueue, which use an array as inner data structure.
//  - LinkedQueue, which use a linked list as inner data structure.
//  - UnsafeQueue, use heap allocated array managed directly by pointers

use std::fmt::Display;

/// TODO doc + FIFO
pub trait Queue {
    type Item;
    fn enqueue(&mut self, val: Self::Item) -> Result<(), QueueErr>;
    fn dequeue(&mut self) -> Option<Self::Item>;
    fn peek_next(&self) -> Option<&Self::Item>;
    fn peek_next_mut(&mut self) -> Option<&mut Self::Item>;
    fn peek(&self, idx: usize) -> Option<&Self::Item>;
    fn peek_mut(&mut self, idx: usize) -> Option<&mut Self::Item>;
    fn is_empty(&self) -> bool;
    fn is_full(&self) -> bool;
}

#[derive(Debug, PartialEq, Eq)]
pub enum QueueErr {
    ZeroCapacityNotAllowed,
    StackOverflow,
}

impl std::error::Error for QueueErr {}

impl Display for QueueErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueueErr::StackOverflow => write!(
                f,
                "Unable to insert a new value in the stack, limit reached"
            ),
            QueueErr::ZeroCapacityNotAllowed => {
                write!(f, "Unable to initialize a stack with no capacity")
            }
        }
    }
}
