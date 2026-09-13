use core::ffi::c_char;

pub type AssertHandler = Option<
  unsafe extern "C-unwind" fn(
    expression: *const c_char,
    file: *const c_char,
    line: i32,
    function: *const c_char,
  ) -> i32,
>;
