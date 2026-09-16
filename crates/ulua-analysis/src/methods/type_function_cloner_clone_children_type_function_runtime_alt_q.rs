use crate::records::{
  type_function_cloner::TypeFunctionCloner,
  type_function_variadic_type_pack::TypeFunctionVariadicTypePack,
};

impl TypeFunctionCloner {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn clone_children_type_function_variadic_type_pack_type_function_variadic_type_pack(
    &mut self,
    v1: *mut TypeFunctionVariadicTypePack,
    v2: *mut TypeFunctionVariadicTypePack,
  ) {
    let source_type = unsafe { (*v1).type_id };
    let target_type = self.shallow_clone_type_function_type_id(source_type);
    unsafe { (*v2).type_id = target_type };
  }
}
