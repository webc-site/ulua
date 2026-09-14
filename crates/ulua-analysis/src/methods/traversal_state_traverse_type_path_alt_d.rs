use crate::records::{reduction::Reduction, traversal_state::TraversalState};

impl TraversalState {
  pub fn traverse_type_path_reduction(&mut self, reduction: Reduction) -> bool {
    if self.check_invariants() {
      return false;
    }
    self.update_current_type_id(reduction.result_type);
    true
  }
}
