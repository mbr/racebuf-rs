//! A racy buffer
//!
//! Implements a `RaceBuf`, which is a buffer that can be read and written
//! to by multiple threads without checking consistency *at all*.
//!
//! Your program will be safe from crashing, but not from partial reads or
//! interleaved writes.
//!
//! `RaceBuf` is best used when it is possible ensure that there are no
//! competing writes (to avoid interleaved writes) and partial reads are
//! not an issue. Types that can be read in a single instruction, like small
//! integers, will likely not suffer from these issues on some platforms.

use std::ptr;

pub struct RaceBuf<T>(Vec<T>);


impl<T: Clone + Copy> RaceBuf<T> {
    #[inline]
    pub fn new_with_value(size: usize, value: T) -> RaceBuf<T> {
        let mut v = Vec::with_capacity(size);
        v.resize(size, value);
        RaceBuf(v)
    }

    #[inline(always)]
    pub fn get(&self, idx: usize) -> Option<T> {
        if idx >= self.0.len() {
            None
        } else {
            Some(unsafe { self.get_unchecked(idx) })
        }
    }

    #[inline(always)]
    pub unsafe fn get_unchecked(&self, idx: usize) -> T {
        // unsafe: bounds are not checked, pointer deref
        ptr::read_volatile(self.0.as_ptr().offset(idx as isize))
    }

    #[inline(always)]
    pub fn set(&self, idx: usize, value: T) {
        if idx >= self.0.len() {
            return;
        } else {
            unsafe { self.set_unchecked(idx, value) }
        }
    }

    #[inline(always)]
    pub unsafe fn set_unchecked(&self, idx: usize, value: T) {
        // unsafe: bounds are not checked, pointer deref, *const T to *mut T
        ptr::write_volatile(self.0.as_ptr().offset(idx as isize) as *mut T, value)
    }
}

impl<T: Clone + Copy + Default> RaceBuf<T> {
    #[inline]
    pub fn new(size: usize) -> RaceBuf<T> {
        RaceBuf::new_with_value(size, T::default())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
