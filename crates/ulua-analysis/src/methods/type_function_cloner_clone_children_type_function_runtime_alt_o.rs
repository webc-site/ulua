use crate::records::{
  type_function_cloner::TypeFunctionCloner, type_function_generic_type::TypeFunctionGenericType,
};

impl TypeFunctionCloner {
  pub fn clone_children_type_function_generic_type_type_function_generic_type(
    &mut self,
    _g1: *mut TypeFunctionGenericType,
    _g2: *mut TypeFunctionGenericType,
  ) {
    // noop.
  }
}
