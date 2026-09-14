use crate::records::{never_type::NeverType, type_cloner::TypeCloner};

impl TypeCloner {
  pub fn clone_children_never_type(&mut self, _t: *mut NeverType) {
    // noop
  }
}
