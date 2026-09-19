use crate::records::{
  any_type::AnyType, type_function_any_type::TypeFunctionAnyType,
  type_function_deserializer::TypeFunctionDeserializer,
};

impl TypeFunctionDeserializer {
  pub fn deserialize_children_type_function_any_type_any_type(
    &mut self,
    _a2: *mut TypeFunctionAnyType,
    _a1: *mut AnyType,
  ) {
    // noop.
  }
}
