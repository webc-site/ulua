use crate::records::{primitive_type::PrimitiveType, type_cloner::TypeCloner};

impl TypeCloner {
  pub fn clone_children_primitive_type(&mut self, _t: *mut PrimitiveType) {
    // noop.
  }
}
