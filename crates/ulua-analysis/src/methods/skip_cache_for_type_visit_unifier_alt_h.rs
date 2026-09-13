use std::ptr::eq;

use crate::{
  records::skip_cache_for_type::SkipCacheForType, type_aliases::type_pack_id::TypePackId,
};
impl SkipCacheForType {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  pub unsafe fn visit_type_pack_id(&mut self, tp: TypePackId) -> bool {
    unsafe {
      if !eq((*tp).owning_arena, self.type_arena) {
        return false;
      }
    }
    true
  }
}
