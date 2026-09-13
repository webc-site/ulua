use core::{
  iter::Rev,
  slice::{Iter, from_raw_parts},
};

use crate::records::ast_array::AstArray;

impl<T> AstArray<T> {
  pub fn rbegin(&self) -> Rev<Iter<'_, T>> {
    unsafe { from_raw_parts(self.data, self.size).iter().rev() }
  }
}
