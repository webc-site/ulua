use crate::records::{any_type::AnyType, type_cloner::TypeCloner};

impl TypeCloner {
  pub fn clone_children_any_type(&mut self, _t: *mut AnyType) {
    // noop.
  }
}
