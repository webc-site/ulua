use std::ptr::eq;

use crate::{records::skip_cache_for_type::SkipCacheForType, type_aliases::type_id::TypeId};
impl SkipCacheForType {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  pub(crate) fn visit_type_id(&mut self, ty: TypeId) -> bool {
    unsafe {
      if !eq((*ty).owning_arena, self.type_arena) {
        return false;
      }
      if let Some(prev) = (*self.skip_cache_for_type).find(&ty)
        && *prev
      {
        self.result = true;
        return false;
      }
    }
    true
  }
}
