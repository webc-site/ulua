use core::ptr::null;

use crate::records::ast_array::AstArray;

impl<T> AstArray<T> {
  pub fn end(&self) -> *const T {
    if self.data.is_null() {
      null()
    } else {
      unsafe { self.data.add(self.size) }
    }
  }
}
