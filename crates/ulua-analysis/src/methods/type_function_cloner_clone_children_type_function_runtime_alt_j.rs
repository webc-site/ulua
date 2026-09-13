use crate::records::{
  type_function_cloner::TypeFunctionCloner,
  type_function_intersection_type::TypeFunctionIntersectionType,
};

impl TypeFunctionCloner {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn clone_children_type_function_intersection_type_type_function_intersection_type(
    &mut self,
    i1: *mut TypeFunctionIntersectionType,
    i2: *mut TypeFunctionIntersectionType,
  ) {
    let components = unsafe { &(*i1).components };
    let target_components = unsafe { &mut (*i2).components };
    for ty in components.iter() {
      target_components.push(self.shallow_clone_type_function_type_id(*ty));
    }
  }
}
