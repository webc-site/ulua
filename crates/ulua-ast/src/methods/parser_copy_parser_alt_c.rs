use core::ptr::null;

use crate::records::{ast_array::AstArray, parser::Parser};

impl Parser {
  pub fn copy_initializer_list_t<T: Clone>(&mut self, data: &[T]) -> AstArray<T> {
    self.copy_t_usize(
      if data.is_empty() {
        null()
      } else {
        data.as_ptr()
      },
      data.len(),
    )
  }
}
