use core::ffi::{c_char, c_void};
pub type LuaCoverage = Option<
  unsafe extern "C-unwind" fn(
    context: *mut c_void,
    function: *const c_char,
    linedefined: i32,
    depth: i32,
    hits: *const i32,
    size: usize,
  ),
>;
