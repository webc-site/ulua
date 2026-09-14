use crate::records::{
  type_function_serializer::TypeFunctionSerializer,
  type_function_unknown_type::TypeFunctionUnknownType, unknown_type::UnknownType,
};

impl TypeFunctionSerializer {
  pub fn serialize_children_unknown_type_type_function_unknown_type(
    &mut self,
    _u1: *const UnknownType,
    _u2: *mut TypeFunctionUnknownType,
  ) {
  }
}
