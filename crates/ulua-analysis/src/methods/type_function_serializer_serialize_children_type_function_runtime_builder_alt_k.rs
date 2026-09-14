use crate::records::{
  negation_type::NegationType, type_function_negation_type::TypeFunctionNegationType,
  type_function_serializer::TypeFunctionSerializer,
};

impl TypeFunctionSerializer {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn serialize_children_negation_type_type_function_negation_type(
    &mut self,
    n1: *const NegationType,
    n2: *mut TypeFunctionNegationType,
  ) {
    unsafe {
      let n1 = &*n1;
      let n2 = &mut *n2;

      n2.type_id = self.shallow_serialize_type_id(n1.ty);
    }
  }
}
