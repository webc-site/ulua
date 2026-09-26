//! Source: `Analysis/src/TypeInfer.cpp:5585-5588` (hand-ported)
//! C++ `TypeId TypeChecker::addTV(Type&& tv) { return currentModule->internalTypes.addType(std::move(tv)); }`.
use crate::{
  functions::arc_as_mut::arc_as_mut,
  records::{r#type::Type, type_checker::TypeChecker},
  type_aliases::type_id::TypeId,
};

impl TypeChecker {
  pub fn add_tv(&mut self, tv: Type) -> TypeId {
    // currentModule->internalTypes.addType(std::move(tv))
    unsafe {
      let module = arc_as_mut(self.current_module.as_ref().expect("current_module"));
      (*module).internal_types.add_tv(tv)
    }
  }
}
