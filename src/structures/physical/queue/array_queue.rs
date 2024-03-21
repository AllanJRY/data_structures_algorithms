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
}

impl<T> Queue for ArrayQueue<T> {
    type Item = T;

    fn enqueue(&mut self, val: Self::Item) -> Result<(), QueueErr> {
        todo!()
    }

    fn dequeue(&mut self) -> Option<Self::Item> {
        todo!()
    }

    fn peek_next(&self) -> Option<&Self::Item> {
        todo!()
    }

    fn peek_next_mut(&mut self) -> Option<&mut Self::Item> {
        todo!()
    }

    fn peek(&self, idx: usize) -> Option<&Self::Item> {
        todo!()
    }

    fn peek_mut(&mut self, idx: usize) -> Option<&mut Self::Item> {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
    }

    fn is_full(&self) -> bool {
        todo!()
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
}
