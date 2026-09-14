use crate::records::{
  type_function_cloner::TypeFunctionCloner, type_function_negation_type::TypeFunctionNegationType,
};

impl TypeFunctionCloner {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn clone_children_type_function_negation_type_type_function_negation_type(
    &mut self,
    n1: *mut TypeFunctionNegationType,
    n2: *mut TypeFunctionNegationType,
  ) {
    let source_type = unsafe { (*n1).type_id };
    let target_type = self.shallow_clone_type_function_type_id(source_type);
    unsafe { (*n2).type_id = target_type };
  }
}
