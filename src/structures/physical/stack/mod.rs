#![allow(dead_code)]
// TODO : implementations :
//  - UnsafeStack, use heap allocated array managed directly by pointers

pub use array_stack::ArrayStack;
pub use linked_stack::LinkedStack;
use std::fmt::Display;

mod array_stack;
mod linked_stack;

/// TODO doc + LIFO
pub trait Stack {
    type Item;
    fn push(&mut self, val: Self::Item) -> Result<(), StackErr>;
    fn pop(&mut self) -> Option<Self::Item>;
    fn peek_next(&self) -> Option<&Self::Item>;
    fn peek_next_mut(&mut self) -> Option<&mut Self::Item>;
    fn peek(&self, idx: usize) -> Option<&Self::Item>;
    fn peek_mut(&mut self, idx: usize) -> Option<&mut Self::Item>;
    fn is_empty(&self) -> bool;
    fn is_full(&self) -> bool;
}

#[derive(Debug, PartialEq, Eq)]
pub enum StackErr {
    ZeroCapacityNotAllowed,
    StackOverflow,
}

impl std::error::Error for StackErr {}

impl Display for StackErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackErr::StackOverflow => write!(
                f,
                "Unable to insert a new value in the stack, limit reached"
            ),
            StackErr::ZeroCapacityNotAllowed => {
                write!(f, "Unable to initialize a stack with no capacity")
            }
        }
    }
}
