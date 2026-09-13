use core::ffi::{c_char, c_void};
pub type PerfLogFn = Option<
  unsafe extern "C-unwind" fn(context: *mut c_void, addr: usize, size: u32, symbol: *const c_char),
>;
