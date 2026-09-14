use crate::records::{intersection_type::IntersectionType, type_cloner::TypeCloner};

impl TypeCloner {
  /// # Safety
  /// 调用方须保证 `t` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn clone_children_intersection_type(&mut self, t: *mut IntersectionType) {
    let parts = unsafe { &mut (*t).parts };
    for ty in parts.iter_mut() {
      *ty = self.shallow_clone_type_id(*ty);
    }
  }
}
