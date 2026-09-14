use crate::records::{
  generic_type::GenericType, type_function_deserializer::TypeFunctionDeserializer,
  type_function_generic_type::TypeFunctionGenericType,
};

impl TypeFunctionDeserializer {
  pub fn deserialize_children_type_function_generic_type_generic_type(
    &mut self,
    _g2: *mut TypeFunctionGenericType,
    _g1: *mut GenericType,
  ) {
  }
}
