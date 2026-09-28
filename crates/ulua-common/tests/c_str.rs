use core::{ffi::c_char, ptr::null};
use std::ffi::CStr;

use ulua_common::functions::c_str::{cstr, cstr_bytes, cstr_cow, with_c_str};

#[test]
fn test_with_c_str_already_nul_terminated() {
  let input = b"hello\0";
  let result = with_c_str(input, |ptr| {
    assert_eq!(ptr, input.as_ptr().cast::<c_char>());
    let cstr = unsafe { CStr::from_ptr(ptr) };
    cstr.to_bytes().to_vec()
  });
  assert_eq!(result, b"hello");
}

#[test]
fn test_with_c_str_small_stack_buffer() {
  let input = b"small_lua_identifier";
  let result = with_c_str(input, |ptr| {
    let cstr = unsafe { CStr::from_ptr(ptr) };
    cstr.to_bytes().to_vec()
  });
  assert_eq!(result, b"small_lua_identifier");
}

#[test]
fn test_with_c_str_large_heap_buffer() {
  let input = vec![b'a'; 256];
  let result = with_c_str(&input, |ptr| {
    let cstr = unsafe { CStr::from_ptr(ptr) };
    cstr.to_bytes().to_vec()
  });
  assert_eq!(result, vec![b'a'; 256]);
}

#[test]
fn test_cstr_bytes_and_cow() {
  let literal = b"constant_str\0";
  let ptr = cstr(literal);
  assert_eq!(unsafe { cstr_bytes(ptr) }, b"constant_str");
  assert_eq!(unsafe { cstr_cow(ptr) }, "constant_str");

  let null_ptr: *const c_char = null();
  assert_eq!(unsafe { cstr_bytes(null_ptr) }, b"");
  assert_eq!(unsafe { cstr_cow(null_ptr) }, "");
}
