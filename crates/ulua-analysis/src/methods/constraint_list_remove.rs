use crate::{
  records::constraint_list::ConstraintList,
  type_aliases::blocked_constraint_id::BlockedConstraintId,
};

impl ConstraintList {
  pub fn remove(&mut self, vertex: BlockedConstraintId) {
    if let Some(entry) = self.present.find_mut(&vertex) {
      // If the entry is true then we also need to decrement the number of
      // entries in the constraint list.
      if *entry {
        self.entries -= 1;
      }
      *entry = false;
    }
  }
}
