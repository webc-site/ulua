use crate::records::{
  singleton_type::SingletonType, type_function_deserializer::TypeFunctionDeserializer,
  type_function_singleton_type::TypeFunctionSingletonType,
};

impl TypeFunctionDeserializer {
  pub fn deserialize_children_type_function_singleton_type_singleton_type(
    &mut self,
    _s2: *mut TypeFunctionSingletonType,
    _s1: *mut SingletonType,
  ) {
    // noop.
  }
}
