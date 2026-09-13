use crate::records::{
  never_type::NeverType, type_function_deserializer::TypeFunctionDeserializer,
  type_function_never_type::TypeFunctionNeverType,
};

impl TypeFunctionDeserializer {
  pub fn deserialize_children_type_function_never_type_never_type(
    &mut self,
    _n2: *mut TypeFunctionNeverType,
    _n1: *mut NeverType,
  ) {
  }
}
