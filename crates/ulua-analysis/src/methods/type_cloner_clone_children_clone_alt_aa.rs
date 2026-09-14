use crate::records::{blocked_type_pack::BlockedTypePack, type_cloner::TypeCloner};

impl TypeCloner {
  pub fn clone_children_blocked_type_pack(&mut self, _t: *mut BlockedTypePack) {
    // TODO: In the new solver, we should ice.
  }
}
