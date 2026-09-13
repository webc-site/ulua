use core::ffi::{c_char, c_int, c_void};
pub type LuaCoverage = Option<
  unsafe extern "C-unwind" fn(
    context: *mut c_void,
    function: *const c_char,
    linedefined: c_int,
    depth: c_int,
    hits: *const c_int,
    size: usize,
  ),
>;
