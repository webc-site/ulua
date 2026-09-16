use crate::records::{pending_expansion_type::PendingExpansionType, type_cloner::TypeCloner};

impl TypeCloner {
  pub fn clone_children_pending_expansion_type(&mut self, _t: *mut PendingExpansionType) {
    // TODO: In the new solver, we should ice.
  }
}
