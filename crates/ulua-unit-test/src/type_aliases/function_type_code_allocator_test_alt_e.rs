use core::ffi::c_void;
pub type FunctionType =
  extern "C-unwind" fn(*mut c_void, Option<extern "C-unwind" fn(i64)>, *mut c_void) -> i64;
