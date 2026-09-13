use core::ffi::{c_int, c_void};
pub type LuaCounterValue =
  Option<unsafe extern "C-unwind" fn(context: *mut c_void, kind: c_int, line: c_int, hits: u64)>;
