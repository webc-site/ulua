use crate::{
  records::constraint_graph::ConstraintGraph,
  type_aliases::blocked_constraint_id::BlockedConstraintId,
};

impl ConstraintGraph {
  pub fn has_strictly_more_than_one_dependency(&mut self, vertex: BlockedConstraintId) -> bool {
    let deps = self.find_dependency_list(vertex);
    unsafe { (*deps.as_ptr()).size() > 1 }
  }
}
