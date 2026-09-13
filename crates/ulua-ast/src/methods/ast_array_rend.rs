use core::{iter::Rev, slice::Iter};

use crate::records::ast_array::AstArray;

impl<T> AstArray<T> {
  pub fn rend(&self) -> Rev<Iter<'_, T>> {
    self.rbegin()
  }
}
