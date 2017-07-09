//! A racy buffer
//!
//! Implements a `RaceBuf`, which is a buffer that can be read and written
//! to by multiple threads without checking consistency *at all*. It uses
//! volatile read/write operations to make it thread "safe".
//!
//! Your program will be safe from crashing, but not from partial reads or
//! interleaved writes.
//!
//! `RaceBuf` is best used when it is possible ensure that there are no
//! competing writes (to avoid interleaved writes) and partial reads are
//! not an issue. Types that can be read in a single instruction, like small
//! integers, will likely not suffer from these issues on some platforms.

use std::ptr;

/// Racy buffer
///
/// Guaranteed not to crash due to memory safety violations on get/set.
///
/// Guarantees almost nothing else, especially that values stored or loaded
/// are actually valid values for type `T`.
pub struct RaceBuf<T>(Vec<T>);

impl<T: Clone + Copy> RaceBuf<T> {
    /// Create a new buffer, initialized with `value`.
    #[inline]
    pub fn new_with_value(size: usize, value: T) -> RaceBuf<T> {
        let mut v = Vec::with_capacity(size);
        v.resize(size, value);
        RaceBuf(v)
    }

    /// Return a pointer to the first element in the buffer
    #[inline]
    pub fn as_ptr(&self) -> *const RaceBuf<T> {
        self.0.as_ptr() as *const RaceBuf<T>
    }

    /// Create a new buffer from existing vector
    #[inline]
    pub fn from_vec(vec: Vec<T>) -> RaceBuf<T> {
        RaceBuf(vec)
    }

    /// Extra inner buffer from RaceBuf
    #[inline]
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }

    /// Get buffer length
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Retrieve value stored at index
    ///
    /// Will return `None` if the index is out-of-bounds.
    #[inline(always)]
    pub fn get(&self, idx: usize) -> Option<T> {
        if idx >= self.0.len() {
            None
        } else {
            Some(unsafe { self.get_unchecked(idx) })
        }
    }

    /// Retrieve value stored at index without bounds checking
    #[inline(always)]
    pub unsafe fn get_unchecked(&self, idx: usize) -> T {
        // unsafe: bounds are not checked, pointer deref
        ptr::read_volatile(self.0.as_ptr().offset(idx as isize))
    }

    /// Set value at index
    ///
    /// If `idx` is out-of-bounds, `set` will have no effect.
    #[inline(always)]
    pub fn set(&self, idx: usize, value: T) {
        if idx >= self.0.len() {
            return;
        } else {
            unsafe { self.set_unchecked(idx, value) }
        }
    }

    /// Set value at index without bounds checking
    ///
    /// Will definately cause undefined behaviour if `idx` is not within
    /// bounds.
    #[inline(always)]
    pub unsafe fn set_unchecked(&self, idx: usize, value: T) {
        // unsafe: bounds are not checked, pointer deref, *const T to *mut T
        ptr::write_volatile(self.0.as_ptr().offset(idx as isize) as *mut T, value)
    }
}

impl<T: Clone + Copy + Default> RaceBuf<T> {
    /// Create new buffer, initialized with default value
    #[inline]
    pub fn new(size: usize) -> RaceBuf<T> {
        RaceBuf::new_with_value(size, T::default())
    }
}
