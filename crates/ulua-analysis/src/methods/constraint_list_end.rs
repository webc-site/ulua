use core::ptr::NonNull;

use crate::records::{constraint_list::ConstraintList, iterator::Iterator};
impl ConstraintList {
  pub fn end(&mut self) -> Iterator {
    Iterator {
      cl: NonNull::new(self as *mut ConstraintList).unwrap(),
      index: self.order.len(),
    }
  }
}
