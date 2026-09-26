use crate::{
  records::constraint_list::ConstraintList,
  type_aliases::blocked_constraint_id::BlockedConstraintId,
};

impl ConstraintList {
  pub fn contains(&self, vertex: BlockedConstraintId) -> bool {
    match self.present.find(&vertex) {
      Some(entry) => *entry,
      None => false,
    }
  }
}
