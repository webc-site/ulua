use core::ffi::{c_char, c_int, c_void};
pub type LuaCounterFunction = Option<
  unsafe extern "C-unwind" fn(context: *mut c_void, function: *const c_char, linedefined: c_int),
>;
