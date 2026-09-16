use core::{cell::UnsafeCell, ffi::c_void, ptr::null_mut};

use crate::type_aliases::perf_log_fn::PerfLogFn;

// Wrapper to permit Sync on UnsafeCell for native-only global state
#[repr(transparent)]
pub(crate) struct SyncCell<T: Copy>(UnsafeCell<T>);

// SAFETY: native-only code, caller must ensure no data races
unsafe impl<T: Copy> Sync for SyncCell<T> {}

impl<T: Copy> SyncCell<T> {
  const fn new(val: T) -> Self {
    Self(UnsafeCell::new(val))
  }
}

pub(crate) static G_PERF_LOG_CONTEXT: SyncCell<*mut c_void> = SyncCell::new(null_mut());
pub(crate) static G_PERF_LOG_FN: SyncCell<PerfLogFn> = SyncCell::new(None);

pub fn set_perf_log(context: *mut c_void, log_fn: PerfLogFn) {
  // SAFETY: native-only context, called before concurrent reads
  unsafe {
    *G_PERF_LOG_CONTEXT.0.get() = context;
    *G_PERF_LOG_FN.0.get() = log_fn;
  }
}
