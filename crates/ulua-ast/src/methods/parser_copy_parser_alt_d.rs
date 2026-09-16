use core::{
  ffi::c_char,
  ptr::{copy_nonoverlapping, null_mut},
};

use crate::records::{allocator::Allocator, ast_array::AstArray, parser::Parser};

impl Parser {
  pub fn copy_string(&mut self, data: &str) -> AstArray<c_char> {
    // C++ `copy(data.c_str(), data.size() + 1)` reads size()+1 bytes because
    // std::string::c_str() is NUL-terminated with a readable size()+1 Buffer.
    // Rust's `String` is NOT NUL-terminated and `String::as_ptr()` is a dangling
    // pointer when the string is empty, so copying `len()+1` bytes from the source
    // over-reads it (a guaranteed SIGSEGV on empty content, UB otherwise). Allocate
    // len+1, copy the `len` content bytes, and write the trailing NUL ourselves so
    // the result keeps c_str() semantics (NUL-terminated, logical size = len).
    let len = data.len();
    let mut result: AstArray<c_char> = AstArray {
      data: null_mut(),
      size: len,
    };

    unsafe {
      let storage = Allocator::allocate(&mut *self.allocator, len + 1) as *mut c_char;

      if len > 0 {
        copy_nonoverlapping(data.as_ptr() as *const c_char, storage, len);
      }
      *storage.add(len) = 0;

      result.data = storage;
    }

    result
  }
}
