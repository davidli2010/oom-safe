use crate::sealed::Sealed;
use std::alloc::{Allocator, Global};
use std::collections::TryReserveError;

/// Extension for `Vec<T, A>`
pub trait VecAllocExt<T, A: Allocator>: Sized + Sealed {
    fn try_with_capacity_in(capacity: usize, alloc: A) -> Result<Self, TryReserveError>;
    fn try_push(&mut self, value: T) -> Result<(), TryReserveError>;
    fn try_resize(&mut self, new_len: usize, value: T) -> Result<(), TryReserveError>
    where
        T: Copy;
    fn try_extend_from_slice(&mut self, other: &[T]) -> Result<(), TryReserveError>
    where
        T: Copy;
}

/// Extension for `Vec<T>`
pub trait VecExt<T>: VecAllocExt<T, Global> {
    fn try_with_capacity(capacity: usize) -> Result<Self, TryReserveError>;
}

impl<T, A: Allocator> Sealed for Vec<T, A> {}

impl<T, A: Allocator> VecAllocExt<T, A> for Vec<T, A> {
    #[inline]
    fn try_with_capacity_in(capacity: usize, alloc: A) -> Result<Self, TryReserveError> {
        let mut vec = Vec::new_in(alloc);
        vec.try_reserve(capacity)?;
        Ok(vec)
    }

    #[inline]
    fn try_push(&mut self, value: T) -> Result<(), TryReserveError> {
        if self.len() == self.capacity() {
            self.try_reserve(1)?;
        }
        self.push(value);
        Ok(())
    }

    #[inline]
    fn try_resize(&mut self, new_len: usize, value: T) -> Result<(), TryReserveError>
    where
        T: Copy,
    {
        if new_len > self.len() {
            self.try_reserve(new_len - self.len())?;
        }
        self.resize(new_len, value);
        Ok(())
    }

    fn try_extend_from_slice(&mut self, other: &[T]) -> Result<(), TryReserveError>
    where
        T: Copy,
    {
        self.try_reserve(other.len())?;
        self.extend_from_slice(other);
        Ok(())
    }
}

impl<T> VecExt<T> for Vec<T> {
    #[inline]
    fn try_with_capacity(capacity: usize) -> Result<Self, TryReserveError> {
        let mut vec = Vec::new();
        vec.try_reserve(capacity)?;
        Ok(vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::alloc::{AllocError, Layout};
    use std::ptr::NonNull;

    struct Alloc(Global);

    impl Alloc {
        pub const fn new() -> Self {
            Alloc(Global)
        }
    }

    unsafe impl Allocator for Alloc {
        fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
            self.0.allocate(layout)
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            self.0.deallocate(ptr, layout)
        }
    }

    #[test]
    fn test_vec() {
        let mut v = Vec::try_with_capacity(1).unwrap();
        v.try_push(1).unwrap();
        v.try_push(2).unwrap();
        v.try_push(3).unwrap();
        assert_eq!(v, [1, 2, 3]);
    }

    #[test]
    fn test_vec_with_alloc() {
        let mut v = Vec::try_with_capacity_in(1, Alloc::new()).unwrap();
        v.try_push(1).unwrap();
        v.try_push(2).unwrap();
        v.try_push(3).unwrap();
        assert_eq!(v, [1, 2, 3]);
    }

    #[test]
    fn test_vec_resize() {
        let mut v = Vec::new_in(Alloc::new());
        v.try_resize(3, 1).unwrap();
        assert_eq!(v, [1, 1, 1]);
    }

    #[test]
    fn test_vec_extend_from_slice() {
        let mut v = Vec::new_in(Alloc::new());
        v.try_extend_from_slice(&[1, 2, 3]).unwrap();
        assert_eq!(v, [1, 2, 3]);
    }
}
