use crate::records::{blocked_type::BlockedType, type_cloner::TypeCloner};

impl TypeCloner {
  pub fn clone_children_blocked_type(&mut self, _t: *mut BlockedType) {
    // TODO: In the new solver, we should ice.
  }
}
