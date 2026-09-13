use crate::records::{generic_type_pack::GenericTypePack, type_cloner::TypeCloner};

impl TypeCloner {
  pub fn clone_children_generic_type_pack(&mut self, _t: *mut GenericTypePack) {
    // TODO: clone upper bounds.
  }
}
