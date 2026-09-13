use crate::records::{
  primitive_type::PrimitiveType, type_function_primitive_type::TypeFunctionPrimitiveType,
  type_function_serializer::TypeFunctionSerializer,
};

impl TypeFunctionSerializer {
  pub fn serialize_children_primitive_type_type_function_primitive_type(
    &mut self,
    _p1: *const PrimitiveType,
    _p2: *mut TypeFunctionPrimitiveType,
  ) {
  }
}
