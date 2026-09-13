use core::ffi::c_int;
pub type FunctionType = extern "C-unwind" fn(i64, *mut c_int) -> i64;
