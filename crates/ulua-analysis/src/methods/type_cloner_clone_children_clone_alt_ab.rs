use crate::{records::type_cloner::TypeCloner, type_aliases::bound_type_pack::BoundTypePack};

impl TypeCloner {
  /// # Safety
  /// 调用方须保证 `t` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn clone_children_bound_type_pack(&mut self, t: *mut BoundTypePack) {
    unsafe {
      (*t).bound_to = self.shallow_clone_type_pack_id((*t).bound_to);
    }
  }
}
