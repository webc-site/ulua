use crate::records::{type_cloner::TypeCloner, variadic_type_pack::VariadicTypePack};

impl TypeCloner {
  /// # Safety
  /// 调用方须保证 `t` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn clone_children_variadic_type_pack(&mut self, t: *mut VariadicTypePack) {
    unsafe {
      (*t).ty = self.shallow_clone_type_id((*t).ty);
    }
  }
}
