use crate::records::traversal_state::TraversalState;

impl TraversalState {
  pub fn check_invariants(&mut self) -> bool {
    self.too_long()
  }
}
