use core::ptr::NonNull;

use crate::records::{constraint_list::ConstraintList, iterator::Iterator};

impl Iterator {
  pub fn iterator(&mut self, cl: NonNull<ConstraintList>, index: usize) {
    self.cl = cl;
    self.index = index;
    self.advance_until_present_or_end();
  }
}
