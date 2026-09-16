use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::follow_type_pack::follow_type_pack_id,
  records::traversal_state::TraversalState,
  type_aliases::{type_or_pack::TypeOrPack, type_pack_id::TypePackId},
};
impl TraversalState {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn update_current_type_pack_id(&mut self, tp: TypePackId) {
    LUAU_ASSERT!(!tp.is_null());
    self.current = TypeOrPack::V1(unsafe { follow_type_pack_id(tp) });
  }
}
