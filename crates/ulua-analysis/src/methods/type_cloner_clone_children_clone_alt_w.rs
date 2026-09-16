use crate::records::{negation_type::NegationType, type_cloner::TypeCloner};

impl TypeCloner {
  /// # Safety
  /// 调用方须保证 `t` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn clone_children_negation_type(&mut self, t: *mut NegationType) {
    unsafe {
      (*t).ty = self.shallow_clone_type_id((*t).ty);
    }
  }
}
