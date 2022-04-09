//! Catch out-of-memory panic.

use crate::AllocError;
use std::alloc::Layout;
use std::panic::{PanicInfo, UnwindSafe};
use std::sync::atomic::{AtomicBool, Ordering};

fn oom_hook(layout: Layout) {
    std::panic::panic_any(AllocError(layout))
}

type PanicHook = Box<dyn Fn(&PanicInfo<'_>) + 'static + Sync + Send>;

fn panic_hook(panic_info: &PanicInfo<'_>) {
    // panic abort except alloc error
    if !panic_info.payload().is::<AllocError>() {
        std::process::abort();
    }
}

/// Invokes a closure, capturing the out-of-memory panic if one occurs.
///
/// This function will return `Ok` with the closure's result if the closure
/// does not panic, and will return `AllocError` if allocation error occurs. The
/// process will abort if other panics occur.
#[inline]
pub fn catch_oom<F: FnOnce() -> R + UnwindSafe, R>(f: F) -> Result<R, AllocError> {
    static SET_HOOK: AtomicBool = AtomicBool::new(false);
    if !SET_HOOK.load(Ordering::Acquire) {
        let hook: PanicHook =
            Box::try_new(panic_hook).map_err(|_| AllocError::new(Layout::new::<PanicHook>()))?;
        std::panic::set_hook(hook);
        std::alloc::set_alloc_error_hook(oom_hook);
        SET_HOOK.store(true, Ordering::Release);
    }

    let result = std::panic::catch_unwind(f);
    match result {
        Ok(r) => Ok(r),
        Err(e) => match e.downcast_ref::<AllocError>() {
            None => {
                unreachable!()
            }
            Some(e) => Err(*e),
        },
    }
}
