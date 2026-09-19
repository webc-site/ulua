use alloc::sync::Arc;

use crate::{
  records::{module::Module, r#type::Type, type_checker::TypeChecker},
  type_aliases::type_id::TypeId,
};

impl TypeChecker {
  pub fn add_type<T>(&mut self, tv: &T) -> TypeId
  where
    T: Clone + Into<Type> + 'static,
  {
    unsafe {
      let module =
        Arc::as_ptr(self.current_module.as_ref().expect("current_module")) as *mut Module;
      (*module).internal_types.add_type(tv.clone())
    }
  }
}
