use crate::records::{
  negation_type::NegationType, type_function_deserializer::TypeFunctionDeserializer,
  type_function_negation_type::TypeFunctionNegationType,
};

impl TypeFunctionDeserializer {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn deserialize_children_type_function_negation_type_negation_type(
    &mut self,
    n2: *mut TypeFunctionNegationType,
    n1: *mut NegationType,
  ) {
    unsafe {
      (*n1).ty = self.shallow_deserialize_type_function_type_id((*n2).type_id);
    }
  }
}
