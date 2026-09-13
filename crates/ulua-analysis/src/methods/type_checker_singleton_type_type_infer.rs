use crate::{records::type_checker::TypeChecker, type_aliases::type_id::TypeId};

impl TypeChecker {
  pub fn singleton_type_bool(&mut self, value: bool) -> TypeId {
    unsafe {
      if value {
        (*self.builtin_types).true_type
      } else {
        (*self.builtin_types).false_type
      }
    }
  }
}
