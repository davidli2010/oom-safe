//! Out-of-memory safe

#![feature(allocator_api)]

mod sealed {
    pub trait Sealed {}
}

mod vec_ext;

pub use self::vec_ext::{VecAllocExt, VecExt};
