use ulua_common::DFInt;

use crate::records::traversal_state::TraversalState;

impl TraversalState {
  pub fn too_long(&mut self) -> bool {
    self.steps += 1;
    self.steps > DFInt::LuauTypePathMaximumTraverseSteps.get()
  }
}
