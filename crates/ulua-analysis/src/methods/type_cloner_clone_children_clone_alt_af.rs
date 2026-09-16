use crate::records::{
  type_cloner::TypeCloner, type_function_instance_type_pack::TypeFunctionInstanceTypePack,
};

impl TypeCloner {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn clone_children_type_function_instance_type_pack(
    &mut self,
    t: *mut TypeFunctionInstanceTypePack,
  ) {
    unsafe {
      for ty in (*t).type_arguments.iter_mut() {
        *ty = self.shallow_clone_type_id(*ty);
      }
      for tp in (*t).pack_arguments.iter_mut() {
        *tp = self.shallow_clone_type_pack_id(*tp);
      }
    }
  }
}
