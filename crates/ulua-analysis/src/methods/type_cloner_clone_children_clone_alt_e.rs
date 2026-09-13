use crate::{records::type_cloner::TypeCloner, type_aliases::bound_type::BoundType};

impl TypeCloner {
  /// # Safety
  /// 调用方须保证 `t` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn clone_children_bound_type(&mut self, t: *mut BoundType) {
    unsafe {
      (*t).bound_to = self.shallow_clone_type_id((*t).bound_to);
    }
  }
}
