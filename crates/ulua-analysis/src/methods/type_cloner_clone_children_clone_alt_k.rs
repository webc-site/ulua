use crate::records::{singleton_type::SingletonType, type_cloner::TypeCloner};

impl TypeCloner {
  pub fn clone_children_singleton_type(&mut self, _t: *mut SingletonType) {
    // noop
  }
}
