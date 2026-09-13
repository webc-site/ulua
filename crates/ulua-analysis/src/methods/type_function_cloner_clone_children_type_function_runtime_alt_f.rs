use crate::records::{
  type_function_cloner::TypeFunctionCloner, type_function_never_type::TypeFunctionNeverType,
};

impl TypeFunctionCloner {
  pub fn clone_children_type_function_never_type_type_function_never_type(
    &mut self,
    _n1: *mut TypeFunctionNeverType,
    _n2: *mut TypeFunctionNeverType,
  ) {
    // noop.
  }
}
