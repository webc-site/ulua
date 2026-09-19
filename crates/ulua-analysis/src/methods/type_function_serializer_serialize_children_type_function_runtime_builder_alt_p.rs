use crate::records::{
  generic_type::GenericType, type_function_generic_type::TypeFunctionGenericType,
  type_function_serializer::TypeFunctionSerializer,
};

impl TypeFunctionSerializer {
  pub fn serialize_children_generic_type_type_function_generic_type(
    &mut self,
    _g1: *const GenericType,
    _g2: *mut TypeFunctionGenericType,
  ) {
  }
}
