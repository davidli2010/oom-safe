//! Out-of-memory safe

#![feature(allocator_api)]

mod sealed {
    pub trait Sealed {}
}

mod vec;

pub use self::vec::{VecAllocExt, VecExt};
