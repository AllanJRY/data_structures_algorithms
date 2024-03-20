#![allow(dead_code)]

use std::cmp::Ordering;
use std::{
    alloc,
    io::Write,
    isize, mem,
    ops::{Deref, DerefMut},
    process::id,
    ptr::NonNull,
};

pub struct Array<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

// TODO: add documentation on attributes and fn, remember to add time complexity
// (with best case and worst case if necessary)
impl<T> Array<T> {
    pub fn new() -> Self {
        // Zero sized type are not yet supported, the main challenge here is
        // to handle the fact that ZST take no space in memory so the pointer
        // will always return the same address, the first one.
        assert_ne!(mem::size_of::<T>(), 0, "Zero sized type not supported");
        Self {
            ptr: NonNull::dangling(),
            cap: 0,
            len: 0,
        }
    }

    pub fn get(&self, idx: usize) -> &T {
        assert!(idx < self.len, "index out of bound");
        unsafe {
            // SAFETY: We are sure that the pointer is not null or it is
            // pointing to an invalid address, because we use an assertion
            // on the idx value at the beginning of the fn.
            return self.ptr.as_ptr().add(idx).as_ref().unwrap();
        }
        // To avoid unsafe, and because we impl Deref en it returns a slice of
        // the array, we can use the brackets syntax, Slices implements Index.
        // return &self[idx];
    }

    pub fn set(&mut self, idx: usize, item: T) {
        assert!(idx < self.len, "index out of bound");
        unsafe {
            // SAFETY: We are sure that the pointer is not null or it is
            // pointing to an invalid address, because we use an assertion
            // on the idx value at the beginning of the fn.
            self.ptr.as_ptr().add(idx).write(item);
        }
        // To avoid unsafe, and because we impl Deref en it returns a slice of
        // the array, we can use the brackets syntax, Slices implements Index.
        // self[idx] = item;
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

    pub fn insert(&mut self, index: usize, item: T) {
        // We need to check if the index is in the bounds, to avoid writing at
        // an address that we don't owned. Note "<=" is used because
        // it's totally ok to add just after the last item, this would behave
        // like a push.
        assert!(index <= self.len, "index out of bounds");
        // We check if we have enough space to add the new item. Otherwise we
        // grow.
        if self.len() == self.cap {
            self.grow();
        }

        unsafe {
            // Here we use the base pointer to point to the address were we
            // want to insert the item, and we shift the current element at
            // that position and all the next by one slot. that way we can
            // write the new item.
            std::ptr::copy(
                self.ptr.as_ptr().add(index),
                self.ptr.as_ptr().add(index + 1),
                self.len - index,
            );
            // existing values have been shifted, we can now wrtie the item
            // safely without taking the risk to overwritte and miss values.
            self.ptr.as_ptr().add(index).write(item);
        }

        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        // We need to check if the index is in the bounds, to avoid writing at
        // an address that we don't owned. Note that here, it is not valid to
        // remove after everything compared to insert.
        assert!(index < self.len, "index out of bounds");
        unsafe {
            // We reduce the len before the shifting, this way we avoid a
            // duplicate minus 1 operation on the len.
            self.len -= 1;
            // We need to read the item now, because the shift will overwrite
            // the value.
            let removed_item = self.ptr.as_ptr().add(index).read();
            // Here we use the base pointer to point to the address which came
            // just after the address at which we want to remove the value,
            // and we shift back by one address all the value which follows
            std::ptr::copy(
                self.ptr.as_ptr().add(index + 1),
                self.ptr.as_ptr().add(index),
                self.len - index,
            );
            removed_item
        }
    }

    // TODO: get + set + IntoIter + Drain + Handling ZST

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, alloc::Layout::array::<T>(1).unwrap())
        } else {
            // every time we need to expand, we double de capacity.
            let new_cap = 2 * self.cap;

            // We get a new memory layout for the new array with the expanded
            // capacity. If the new size exceed isize::MAX, the array function
            // will fail, so we panic because the program reach it's space
            // limit.
            match alloc::Layout::array::<T>(new_cap) {
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
            // IMPORTANTE NOTE: It is important to give the new layout
            // size to the size arg, because a bug may occure when the program
            // sometimes access an invalid memory address (on not owned) when
            // self.cap is used. The layout size is calculated with size and
            // alignement took into account. Note to me: when allocating manually
            // always use layout infos.
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) as *mut T }
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

impl<T: Eq> Array<T> {
    // add to doc :
    // We can improve linear search in 2 ways :
    // The first would be to use transposition, every time an element is
    // searched and found, we move this element closer to the first index,
    // this way frequently searched element will be faster and faster,
    // The second one is **move to front/head**, the idea is the same as before
    // but we always swap with the element at index 0.
    // For both solutions, we try to move the element frequently searched
    // element to the first index, to get a constant time look up.
    pub fn linear_search(&self, other: &T) -> Option<usize> {
        for (idx, v) in self.iter().enumerate() {
            if v == other {
                return Some(idx);
            }
        }
        None
    }
}

impl<T: Eq + Ord> Array<T> {
    pub fn binary_search(&self, other: &T) -> Option<usize> {
        if self.len == 0 {
            return None;
        }

        let mut low_idx = 0usize;
        let mut high_idx = self.len - 1;

        while low_idx <= high_idx {
            let mid_idx = (low_idx + high_idx) / 2;
            dbg!(low_idx, mid_idx, high_idx);
            match other.cmp(&self[mid_idx]) {
                Ordering::Equal => return Some(mid_idx),
                Ordering::Less => high_idx = mid_idx - 1,
                Ordering::Greater => low_idx = mid_idx + 1,
            }
        }
        None
    }

    pub fn push_sorted(&mut self, item: T) {
        // We start by considering that the new value is greater than every
        // other, so it will be added at the end.
        let mut insert_idx = self.len;
        // We find the right index if there is greater values in the array
        for (idx, _) in self.iter().enumerate() {
            if item < self[idx] {
                // Once we found a greater element, we take his index,
                // it will be shifted to the left.
                insert_idx = idx;
                break;
            }
        }

        // We insert at the defined index, all the element on the left will be
        // shifted.
        self.insert(insert_idx, item);
    }

    pub fn is_sorted(&self) -> bool {
        for (idx, val) in self.iter().enumerate() {
            if idx + 1 >= self.len {
                break;
            }

            if val > self.get(idx + 1) {
                return false;
            }
        }
        true
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
    use std::ops::Index;

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
    fn array_get() {
        let mut arr = Array::<i32>::new();
        arr.push(1);
        arr.push(3);
        arr.push(7);
        assert_eq!(&7, arr.get(2));
        assert_eq!(&1, arr.get(0));
        assert_eq!(&7, arr.get(2));
    }

    #[test]
    fn array_set() {
        let mut arr = Array::<i32>::new();
        arr.push(1);
        arr.push(3);
        arr.push(7);
        arr.set(0, 9);
        assert_eq!(&9, arr.get(0));
        assert_eq!(&7, arr.get(2));
        arr.set(2, 11);
        assert_eq!(&11, arr.get(2));
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
        assert_eq!(4, arr.cap);
        assert_eq!(2, arr.len);
        assert_eq!(Some(3), arr.pop());
        assert_eq!(4, arr.cap);
        assert_eq!(1, arr.len);
        assert_eq!(Some(1), arr.pop());
        assert_eq!(None, arr.pop());
    }

    #[test]
    fn array_insert() {
        let mut arr = Array::<i32>::new();
        arr.push(1);
        arr.push(3);
        arr.push(7);
        arr.insert(1, 9);
        assert_eq!(4, arr.cap);
        assert_eq!(4, arr.len);
        assert_eq!(9, arr[1]);
        assert_eq!(3, arr[2]);
    }

    #[test]
    fn array_insert_tail() {
        let mut arr = Array::<i32>::new();
        arr.push(1);
        arr.push(3);
        arr.push(5);
        arr.push(7);
        arr.insert(arr.len, 9);
        assert_eq!(8, arr.cap);
        assert_eq!(5, arr.len);
        assert_eq!(9, arr[arr.len - 1]);
    }

    #[test]
    fn array_remove() {
        let mut arr = Array::<i32>::new();
        arr.push(1);
        arr.push(3);
        arr.push(7);
        assert_eq!(3, arr.remove(1));
        assert_eq!(4, arr.cap);
        assert_eq!(2, arr.len);
        assert_eq!(7, arr[1]);
    }

    #[test]
    fn array_remove_tail() {
        let mut arr = Array::<i32>::new();
        arr.push(1);
        arr.push(3);
        arr.push(7);
        assert_eq!(7, arr.remove(arr.len - 1));
        assert_eq!(4, arr.cap);
        assert_eq!(2, arr.len);
        assert_eq!(3, arr[arr.len - 1]);
    }

    #[test]
    fn array_at_index() {
        let mut arr = Array::<i32>::new();
        arr.push(1);
        assert_eq!(1, arr.cap);
        assert_eq!(1, arr.len);
        assert_eq!(1, arr[0]);
    }

    #[test]
    fn array_linear_search() {
        let mut arr = Array::<i32>::new();
        arr.push(1);
        arr.push(3);
        arr.push(5);
        arr.push(7);
        assert_eq!(Some(3), arr.linear_search(&7));
        assert_eq!(None, arr.linear_search(&9));
    }

    #[test]
    fn array_binary_search() {
        let mut arr = Array::<i32>::new();
        arr.push(1);
        arr.push(3);
        arr.push(5);
        arr.push(7);
        arr.push(9);
        arr.push(11);
        arr.push(12);
        assert_eq!(Some(3), arr.binary_search(&7));
        assert_eq!(Some(1), arr.binary_search(&3));
        assert_eq!(Some(6), arr.binary_search(&12));
        assert_eq!(None, arr.binary_search(&13))
    }

    #[test]
    fn array_push_sorted() {
        let mut arr = Array::<i32>::new();
        arr.push_sorted(1);
        arr.push_sorted(3);
        arr.push_sorted(5);
        arr.push_sorted(7);
        arr.push_sorted(9);
        arr.push_sorted(11);
        arr.push_sorted(12);
        assert_eq!(&7, arr.get(3));
        assert_eq!(&11, arr.get(5));
        arr.push_sorted(6);
        assert_eq!(&6, arr.get(3));
        assert_eq!(&7, arr.get(4));
        assert_eq!(&11, arr.get(6));
        arr.push_sorted(13);
        assert_eq!(&13, arr.get(arr.len - 1));
    }

    #[test]
    fn array_is_sorted() {
        let mut arr = Array::<i32>::new();
        arr.push(1);
        arr.push(3);
        arr.push(7);
        assert!(arr.is_sorted());
        let mut unsorted_arr = Array::<i32>::new();
        unsorted_arr.push(1);
        unsorted_arr.push(7);
        unsorted_arr.push(3);
        assert!(!unsorted_arr.is_sorted());
    }
}
