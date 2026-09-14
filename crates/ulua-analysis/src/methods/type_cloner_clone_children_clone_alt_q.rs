use crate::records::{no_refine_type::NoRefineType, type_cloner::TypeCloner};

impl TypeCloner {
  pub fn clone_children_no_refine_type(&mut self, _t: *mut NoRefineType) {
    // noop.
  }
}
