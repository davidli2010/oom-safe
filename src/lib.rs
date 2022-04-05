//! Out-of-memory safe

#![feature(allocator_api)]
#![feature(alloc_error_hook)]
#![feature(try_reserve_kind)]

use std::alloc::Layout;
use std::collections::TryReserveError;
use std::error::Error;
use std::fmt;

mod sealed {
    pub trait Sealed {}
}

mod oom;
mod vec_ext;

pub use crate::oom::catch_oom;
pub use crate::vec_ext::{VecAllocExt, VecExt};

/// The error type for allocation failure.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct AllocError(Layout);

impl AllocError {
    /// Creates a new `AllocError`.
    #[must_use]
    #[inline]
    pub const fn new(layout: Layout) -> Self {
        AllocError(layout)
    }

    /// Returns the memory layout of the `AllocError`.
    #[must_use]
    #[inline]
    pub const fn layout(self) -> Layout {
        self.0
    }
}

impl fmt::Debug for AllocError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AllocError")
            .field("size", &self.0.size())
            .field("align", &self.0.align())
            .finish()
    }
}

impl fmt::Display for AllocError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "failed to allocate memory by required layout {{size: {}, align: {}}}",
            self.0.size(),
            self.0.align()
        )
    }
}

impl Error for AllocError {}

impl From<TryReserveError> for AllocError {
    #[inline]
    fn from(e: TryReserveError) -> Self {
        use std::collections::TryReserveErrorKind;
        match e.kind() {
            TryReserveErrorKind::AllocError { layout, .. } => AllocError::new(layout),
            TryReserveErrorKind::CapacityOverflow => {
                unreachable!("unexpected capacity overflow")
            }
        }
    }
}
