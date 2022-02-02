use crate::alloc::{Allocator, TryReserveError};
use std::ops::Deref;

#[repr(transparent)]
pub struct Vec<T, A: Allocator>(std::vec::Vec<T, A>);

impl<T, A: Allocator> Deref for Vec<T, A> {
    type Target = std::vec::Vec<T, A>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, A: Allocator> AsRef<std::vec::Vec<T, A>> for Vec<T, A> {
    #[inline]
    fn as_ref(&self) -> &std::vec::Vec<T, A> {
        &self.0
    }
}

impl<T, A: Allocator> Vec<T, A> {
    #[inline]
    pub fn new_in(alloc: A) -> Self {
        Vec(std::vec::Vec::new_in(alloc))
    }

    #[inline]
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.0.try_reserve(additional)
    }

    #[inline]
    pub fn try_with_capacity_in(capacity: usize, alloc: A) -> Result<Self, TryReserveError> {
        let mut vec = Vec::new_in(alloc);
        vec.try_reserve(capacity)?;
        Ok(vec)
    }

    #[inline]
    pub fn try_push(&mut self, value: T) -> Result<(), TryReserveError> {
        if self.len() == self.capacity() {
            self.try_reserve(1)?;
        }
        self.0.push(value);
        Ok(())
    }

    #[inline]
    pub fn try_resize_with<F>(&mut self, new_len: usize, f: F) -> Result<(), TryReserveError>
    where
        F: FnMut() -> T,
    {
        if new_len > self.len() {
            self.try_reserve(new_len - self.len())?;
        }
        self.0.resize_with(new_len, f);
        Ok(())
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.0.truncate(len);
    }

    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// # Safety
    ///
    /// - `new_len` must be less than or equal to [`capacity()`].
    /// - The elements at `old_len..new_len` must be initialized.
    #[inline]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        self.0.set_len(new_len)
    }
}
