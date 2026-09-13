//! Source: `Analysis/src/TypePath.cpp:999-1006` (hand-ported)
use crate::{
  functions::{follow_type::follow_type_id, traverse_type_path::traverse as traverse_path},
  records::{
    builtin_types::BuiltinTypes, path::Path, traversal_state::TraversalState, type_arena::TypeArena,
  },
  type_aliases::{type_id::TypeId, type_or_pack::TypeOrPack},
};

pub fn traverse(
  root: TypeId,
  path: &Path,
  builtin_types: &BuiltinTypes,
  arena: &mut TypeArena,
) -> Option<TypeOrPack> {
  let mut state = TraversalState::traversal_state_type_id_not_null_builtin_types_type_arena(
    follow_type_id(root),
    builtin_types,
    arena,
  );
  if traverse_path(&mut state, path) {
    Some(state.current)
  } else {
    None
  }
}
