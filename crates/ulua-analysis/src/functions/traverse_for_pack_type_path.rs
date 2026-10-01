use crate::{
  functions::{follow_type, get_type_or_pack::get_type_or_pack_mut, traverse_type_path::traverse},
  records::{
    builtin_types::BuiltinTypes, path::Path, traversal_state::TraversalState, type_arena::TypeArena,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
pub fn traverse_for_pack(
  root: TypeId,
  path: &Path,
  builtin_types: &BuiltinTypes,
  arena: &mut TypeArena,
) -> Option<TypePackId> {
  let mut state = TraversalState::traversal_state_type_id_not_null_builtin_types_type_arena(
    follow_type::follow(root),
    builtin_types,
    arena,
  );
  if traverse(&mut state, path) {
    if state.encountered_error_suppression {
      return Some(builtin_types.error_type_pack);
    }
    if let Some(ty) = get_type_or_pack_mut::<TypePackId>(&state.current).copied() {
      return Some(ty);
    }
  }
  None
}
