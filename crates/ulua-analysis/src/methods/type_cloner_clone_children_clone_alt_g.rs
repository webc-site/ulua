use crate::records::{generic_type::GenericType, type_cloner::TypeCloner};

impl TypeCloner {
  pub fn clone_children_generic_type(&mut self, _t: *mut GenericType) {
    // TODO: clone upper bounds.
  }
}
