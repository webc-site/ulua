use alloc::string::String;
use core::ffi::c_void;
pub type AnnotatorFn = Option<
  unsafe extern "C-unwind" fn(context: *mut c_void, result: &mut String, fid: i32, instpos: i32),
>;
