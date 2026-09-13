use crate::records::{free_type_pack::FreeTypePack, type_cloner::TypeCloner};

impl TypeCloner {
  pub fn clone_children_free_type_pack(&mut self, _t: *mut FreeTypePack) {
    // TODO: clone lower and upper bounds.
    // TODO: In the new solver, we should ice.
  }
}
