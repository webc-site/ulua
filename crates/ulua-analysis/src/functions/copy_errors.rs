use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{
  functions::copy_error::visit_error_data,
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, clone_state::CloneState,
    type_arena::TypeArena,
  },
  type_aliases::error_vec::ErrorVec,
};
pub fn copy_errors(
  errors: &mut ErrorVec,
  dest_arena: &mut TypeArena,
  builtin_types: &BuiltinTypes,
) {
  let mut clone_state = CloneState {
    builtin_types: Handle::from_ref(builtin_types),
    seen_types: DenseHashMap::default(),
    seen_type_packs: DenseHashMap::default(),
  };

  let types_arena = &dest_arena.types;
  let type_packs_arena = &dest_arena.type_packs;
  LUAU_ASSERT!(!types_arena.is_frozen());
  LUAU_ASSERT!(!type_packs_arena.is_frozen());

  for error in errors {
    // 各变体转发规则同构，统一走 visitErrorData 派发（65 臂 match 已在
    // copy_error::visit_error_data 内宏化收口）。
    visit_error_data(&mut error.data, dest_arena, &mut clone_state);
  }
}
