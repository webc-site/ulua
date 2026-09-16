use crate::records::constraint_list::ConstraintList;

impl ConstraintList {
  pub fn size(&self) -> usize {
    self.entries
  }
}
