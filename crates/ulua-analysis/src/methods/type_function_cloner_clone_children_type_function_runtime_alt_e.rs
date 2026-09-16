use crate::records::{
  type_function_cloner::TypeFunctionCloner, type_function_unknown_type::TypeFunctionUnknownType,
};

impl TypeFunctionCloner {
  pub fn clone_children_type_function_unknown_type_type_function_unknown_type(
    &mut self,
    _u1: *mut TypeFunctionUnknownType,
    _u2: *mut TypeFunctionUnknownType,
  ) {
    // noop.
  }
}
