use crate::records::unifier_shared_state::UnifierSharedState;

impl UnifierSharedState {
  pub fn set_recursion_limit(&mut self, recursion_limit: i32) {
    self.counters.recursion_limit = recursion_limit;
  }
}
