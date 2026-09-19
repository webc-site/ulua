use crate::records::{metatable_type::MetatableType, type_cloner::TypeCloner};

impl TypeCloner {
  /// # Safety
  /// 调用方须保证 `t` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn clone_children_metatable_type(&mut self, t: *mut MetatableType) {
    unsafe {
      (*t).table = self.shallow_clone_type_id((*t).table);
      (*t).metatable = self.shallow_clone_type_id((*t).metatable);
    }
  }
}
