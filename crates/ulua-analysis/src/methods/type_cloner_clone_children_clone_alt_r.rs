use crate::records::{type_cloner::TypeCloner, union_type::UnionType};

impl TypeCloner {
  /// # Safety
  /// 调用方须保证 `t` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn clone_children_union_type(&mut self, t: *mut UnionType) {
    unsafe {
      for ty in &mut (*t).options {
        *ty = self.shallow_clone_type_id(*ty);
      }
    }
  }
}
