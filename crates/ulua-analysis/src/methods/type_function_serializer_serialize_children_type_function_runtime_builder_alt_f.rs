use crate::records::{
  never_type::NeverType, type_function_never_type::TypeFunctionNeverType,
  type_function_serializer::TypeFunctionSerializer,
};

impl TypeFunctionSerializer {
  pub fn serialize_children_never_type_type_function_never_type(
    &mut self,
    _n1: *const NeverType,
    _n2: *mut TypeFunctionNeverType,
  ) {
  }
}
