//! Source: `Analysis/src/TypeInfer.cpp:5585-5588` (hand-ported)
//! C++ `TypeId TypeChecker::addTV(Type&& tv) { return currentModule->internalTypes.addType(std::move(tv)); }`.
use alloc::sync::Arc;

use crate::{
  records::{module::Module, r#type::Type, type_checker::TypeChecker},
  type_aliases::type_id::TypeId,
};

impl TypeChecker {
  pub fn add_tv(&mut self, tv: Type) -> TypeId {
    // currentModule->internalTypes.addType(std::move(tv))
    unsafe {
      let module =
        Arc::as_ptr(self.current_module.as_ref().expect("current_module")) as *mut Module;
      (*module).internal_types.add_tv(tv)
    }
  }
}
