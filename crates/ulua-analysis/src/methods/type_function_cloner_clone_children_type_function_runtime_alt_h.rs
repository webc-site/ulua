use crate::records::{
  type_function_cloner::TypeFunctionCloner, type_function_singleton_type::TypeFunctionSingletonType,
};

impl TypeFunctionCloner {
  pub fn clone_children_type_function_singleton_type_type_function_singleton_type(
    &mut self,
    _s1: *mut TypeFunctionSingletonType,
    _s2: *mut TypeFunctionSingletonType,
  ) {
    // noop.
  }
}
