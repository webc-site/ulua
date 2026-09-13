use crate::records::{free_type::FreeType, type_cloner::TypeCloner};

impl TypeCloner {
  /// # Safety
  /// 调用方须保证 `t` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn clone_children_free_type(&mut self, t: *mut FreeType) {
    unsafe {
      if !(*t).lower_bound.is_null() {
        (*t).lower_bound = self.shallow_clone_type_id((*t).lower_bound);
      }
      if !(*t).upper_bound.is_null() {
        (*t).upper_bound = self.shallow_clone_type_id((*t).upper_bound);
      }
    }
  }
}
