#![allow(dead_code)]

use std::{
    alloc,
    io::Write,
    isize, mem,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

pub struct Array<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

impl<T> Array<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            cap: 0,
            len: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        // If are current length is same as our capacity, it means that our
        // array is full need more space to store futher items, so we grow.
        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            // To add the item to the array, we need to write his the tail
            // of our array, so we move to this address starting from the bese
            // pointer which point the the first address. then we write.
            self.ptr.as_ptr().add(self.len).write(item);
            // We don't use the following code, because deref may read before
            // writing, and there is a lot of chance that the value pointed is
            // null and we don't want to read that null value.
            // (DO NOT UNCOMMENT) *self.ptr.as_ptr().add(self.len) = item;
        }

        // We have a new item in the array, so we increment the length.
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            // We start by decrementing the length, this way when we move the
            // pointer to the location want to read, we just add self.len.
            self.len -= 1;
            // By using the function read from the pointer, we just copy the
            // bytes at the given address and returning them, so nothing is
            // moved or deallocated, a valid value of type T will still there.
            // The data will be deallocated when drop is called, for the moment
            // because the array doesn't shrink the data should only be
            // overwritten by this array.
            unsafe { Some(self.ptr.as_ptr().add(self.len).read()) }
        }
    }

    // TODO: insert + remove + IntoIter + Drain + Handling ZST

    fn grow(&mut self) {
        // Zero sized type are not yet supported, the main challenge here is
        // to handle the fact that ZST take no space in memory so the pointer
        // will always return the same address, the first one.
        assert_ne!(mem::size_of::<T>(), 0, "Zero sized type not supported");

        let (new_cap, new_layout) = if self.cap == 0 {
            (1, alloc::Layout::array::<T>(1).unwrap())
        } else {
            // every time we need to expand, we double de capacity.
            let new_cap = 2 * self.cap;

            // We get a new memory layout for the new array with the expanded
            // capacity. If the new size exceed isize::MAX, the array function
            // will fail, so we panic because the program reach it's space
            // limit.
            match alloc::Layout::array::<T>(2 * self.cap) {
                Ok(layout) => (new_cap, layout),
                Err(_) => panic!("Not enough space to allocate more for the array."),
            }
        };

        // Here we need a new pointer which point to the new base address of
        // the newly allocated layout.
        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) as *mut T }
        } else {
            // We reallocate, so we need the old/current layout and the
            // old/current base address pointer to "move" the bytes to another
            // location with more contigeous space for the array.
            // SAFETY: Here we can unwrap, this can't fail because we already
            // succeed to allocate this layout.
            let old_layout = alloc::Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_cap) as *mut T }
        };

        self.cap = new_cap;
        self.ptr = match NonNull::new(new_ptr) {
            Some(ptr) => ptr,
            None => {
                // If the pointer is null, it means that there were an error
                // during allocation, by calling this function, we interrupt
                // the program and try to avoid any new allocation.
                alloc::handle_alloc_error(new_layout)
            }
        }
    }
}

// By implementing Deref and returning a slice of the complete array, we get
// a lot of slice operations for free.
impl<T> Deref for Array<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        // by using from from_raw_parts, we can use the array base pointer and
        // the current length to define the slice range.
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

// By implementing DerefMut and returning a slice of the complete array, we get
// a lot of slice operations for free.
impl<T> DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        // by using from from_raw_parts_mut, we can use the array base pointer
        // and the current length to define the slice range.
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

// We implement Drop here to be sure to not leak lot of resources.
impl<T> Drop for Array<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            while self.pop().is_some() {}
            // We need the current array layout, to give to the global
            // allocator the array reserved space to deallocate.
            // Note, that it is totaly ok to unwrap here, it cannot fail,
            // this is the same layout use to grow.
            let layout = alloc::Layout::array::<T>(self.cap).unwrap();

            // Here we give the pointer to the base address and the layout
            // this way the global allocator know which part of the memory to
            // deallocate.
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_empty_array() {
        let arr = Array::<i32>::new();
        assert_eq!(0, arr.cap);
        assert_eq!(0, arr.len);
    }

    #[test]
    fn grow_array() {
        let mut arr = Array::<i32>::new();
        assert_eq!(0, arr.cap);
        assert_eq!(0, arr.len);
        arr.grow();
        assert_eq!(1, arr.cap);
        assert_eq!(0, arr.len);
        arr.grow();
        assert_eq!(2, arr.cap);
        assert_eq!(0, arr.len);
        arr.grow();
        assert_eq!(4, arr.cap);
        assert_eq!(0, arr.len);
    }

    #[test]
    fn array_push() {
        let mut arr = Array::<i32>::new();
        arr.push(1);
        assert_eq!(1, arr.cap);
        assert_eq!(1, arr.len);
        arr.push(3);
        assert_eq!(2, arr.cap);
        assert_eq!(2, arr.len);
        arr.push(7);
        assert_eq!(4, arr.cap);
        assert_eq!(3, arr.len);
    }

    #[test]
    fn array_pop() {
        let mut arr = Array::<i32>::new();
        assert_eq!(None, arr.pop());
        arr.push(1);
        assert_eq!(1, arr.cap);
        assert_eq!(1, arr.len);
        arr.push(3);
        assert_eq!(2, arr.cap);
        assert_eq!(2, arr.len);
        arr.push(7);
        assert_eq!(4, arr.cap);
        assert_eq!(3, arr.len);
        assert_eq!(Some(7), arr.pop());
        assert_eq!(Some(3), arr.pop());
        assert_eq!(Some(1), arr.pop());
        assert_eq!(None, arr.pop());
    }

    #[test]
    fn array_at_index() {
        let mut arr = Array::<i32>::new();
        arr.push(1);
        assert_eq!(1, arr.cap);
        assert_eq!(1, arr.len);
        assert_eq!(1, arr[0]);
    }
}
