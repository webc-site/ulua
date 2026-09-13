use crate::records::{
  type_function_cloner::TypeFunctionCloner, type_function_union_type::TypeFunctionUnionType,
};

impl TypeFunctionCloner {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn clone_children_type_function_union_type_type_function_union_type(
    &mut self,
    u1: *mut TypeFunctionUnionType,
    u2: *mut TypeFunctionUnionType,
  ) {
    let components = unsafe { &(*u1).components };
    let target_components = unsafe { &mut (*u2).components };
    for ty in components.iter() {
      target_components.push(self.shallow_clone_type_function_type_id(*ty));
    }
  }
}
