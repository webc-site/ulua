use crate::records::{
  singleton_type::SingletonType, type_function_serializer::TypeFunctionSerializer,
  type_function_singleton_type::TypeFunctionSingletonType,
};

impl TypeFunctionSerializer {
  pub fn serialize_children_singleton_type_type_function_singleton_type(
    &mut self,
    _s1: *const SingletonType,
    _s2: *mut TypeFunctionSingletonType,
  ) {
  }
}
