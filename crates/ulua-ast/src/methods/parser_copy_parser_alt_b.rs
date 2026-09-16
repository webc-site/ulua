use core::ptr::null_mut;

use crate::records::{ast_array::AstArray, parser::Parser, temp_vector::TempVector};

impl Parser {
  pub fn copy_temp_vector_t<'a, T: Clone>(&mut self, data: &TempVector<'a, T>) -> AstArray<T> {
    if data.size_ == 0 {
      self.copy_t_usize(null_mut(), 0)
    } else {
      unsafe {
        let ptr = (*data.storage).as_ptr().add(data.offset);
        self.copy_t_usize(ptr as *mut T, data.size_)
      }
    }
  }
}
