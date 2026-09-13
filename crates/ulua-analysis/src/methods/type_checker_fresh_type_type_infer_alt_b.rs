//! @interface-stub
use alloc::sync::Arc;

use crate::{
  records::{module::Module, type_checker::TypeChecker, type_level::TypeLevel},
  type_aliases::type_id::TypeId,
};
impl TypeChecker {
  pub fn fresh_type_type_level(&mut self, level: TypeLevel) -> TypeId {
    unsafe {
      let module = Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module;
      (*module)
        .internal_types
        .fresh_type_not_null_builtin_types_type_level(&*self.builtin_types, level)
    }
  }
}
