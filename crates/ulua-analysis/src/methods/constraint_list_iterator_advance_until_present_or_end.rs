use crate::records::{constraint_list::ConstraintList, iterator::Iterator};

impl Iterator {
  pub fn advance_until_present_or_end(&mut self) {
    while self.index < unsafe { self.cl.as_ref().order.len() }
      && !unsafe {
        ConstraintList::contains(self.cl.as_ref(), self.cl.as_ref().order[self.index].clone())
      }
    {
      self.index += 1;
    }
  }
}
