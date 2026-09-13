use crate::records::{type_cloner::TypeCloner, type_pack::TypePack};

impl TypeCloner {
  /// # Safety
  /// 调用方须保证 `t` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn clone_children_type_pack(&mut self, t: *mut TypePack) {
    unsafe {
      for ty in (*t).head.iter_mut() {
        *ty = self.shallow_clone_type_id(*ty);
      }
      if let Some(tail) = (*t).tail {
        (*t).tail = Some(self.shallow_clone_type_pack_id(tail));
      }
    }
  }
}
