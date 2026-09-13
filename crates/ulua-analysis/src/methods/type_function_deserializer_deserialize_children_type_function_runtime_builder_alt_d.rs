use crate::records::{
  primitive_type::PrimitiveType, type_function_deserializer::TypeFunctionDeserializer,
  type_function_primitive_type::TypeFunctionPrimitiveType,
};

impl TypeFunctionDeserializer {
  pub fn deserialize_children_type_function_primitive_type_primitive_type(
    &mut self,
    _p2: *mut TypeFunctionPrimitiveType,
    _p1: *mut PrimitiveType,
  ) {
  }
}
