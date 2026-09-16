use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{records::anyification::Anyification, type_aliases::type_pack_id::TypePackId};

impl Anyification {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn clean_type_pack_id(&mut self, tp: TypePackId) -> TypePackId {
    LUAU_ASSERT!(unsafe { self.is_dirty_type_pack_id(tp) });
    self.any_type_pack
  }
}
