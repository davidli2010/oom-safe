//! Out-of-memory safe

#![feature(allocator_api)]

mod sealed {
    pub trait Sealed {}
}

mod vec_ext;

pub use crate::vec_ext::{VecAllocExt, VecExt};

pub mod alloc {
    pub use std::alloc::{AllocError, Allocator, Global, Layout, LayoutError};
    pub use std::collections::TryReserveError;
}
