use crate::{
  functions::follow_type_pack::follow_type_pack_id,
  records::{builtin_types::BuiltinTypes, traversal_state::TraversalState, type_arena::TypeArena},
  type_aliases::{type_or_pack::TypeOrPack, type_pack_id::TypePackId},
};
impl TraversalState {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn traversal_state_type_pack_id_not_null_builtin_types_type_arena(
    root: TypePackId,
    builtin_types: &BuiltinTypes,
    arena: &mut TypeArena,
  ) -> Self {
    TraversalState {
      current: TypeOrPack::V1(unsafe { follow_type_pack_id(root) }),
      builtin_types,
      arena,
      steps: 0,
      encountered_error_suppression: false,
    }
  }
}
