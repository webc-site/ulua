use core::ffi::{c_char, c_void};
pub type UserdataRemapperCallback =
  unsafe extern "C-unwind" fn(context: *mut c_void, name: *const c_char, name_length: usize) -> u8;
