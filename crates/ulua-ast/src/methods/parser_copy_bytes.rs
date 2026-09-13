use core::{
  ffi::c_char,
  ptr::{copy_nonoverlapping, null_mut},
};

use crate::records::{allocator::Allocator, ast_array::AstArray, parser::Parser};

impl Parser {
  pub fn copy_bytes(&mut self, data: &[u8]) -> AstArray<c_char> {
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
