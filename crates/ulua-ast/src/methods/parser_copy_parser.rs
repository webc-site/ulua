use core::{
  mem::size_of,
  ptr::{null_mut, write},
  slice::from_raw_parts,
};

use crate::records::{allocator::Allocator, ast_array::AstArray, parser::Parser};

impl Parser {
  pub(crate) fn copy_t_usize<T: Clone>(&mut self, data: *const T, size: usize) -> AstArray<T> {
    let mut result = AstArray {
      data: null_mut(),
      size,
    };

    if size == 0 || data.is_null() {
      return result;
    }

    unsafe {
      let storage = Allocator::allocate(&mut *self.allocator, size_of::<T>() * size) as *mut T;

      result.data = storage;

      let src_slice = from_raw_parts(data, size);
      for (i, src) in src_slice.iter().enumerate() {
        write(result.data.add(i), src.clone());
      }
    }

    result
  }
}
