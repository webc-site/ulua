use crate::records::{
  type_function_deserializer::TypeFunctionDeserializer,
  type_function_unknown_type::TypeFunctionUnknownType, unknown_type::UnknownType,
};

impl TypeFunctionDeserializer {
  pub fn deserialize_children_type_function_unknown_type_unknown_type(
    &mut self,
    _u2: *mut TypeFunctionUnknownType,
    _u1: *mut UnknownType,
  ) {
  }
}
