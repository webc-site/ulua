use crate::{
  functions::{
    follow_type_pack::follow_type_pack_id, traverse_type_path::traverse as traverse_path,
  },
  records::{
    builtin_types::BuiltinTypes, path::Path, traversal_state::TraversalState, type_arena::TypeArena,
  },
  type_aliases::{type_or_pack::TypeOrPack, type_pack_id::TypePackId},
};

pub(crate) fn traverse(
  root: TypePackId,
  path: &Path,
  builtin_types: &BuiltinTypes,
  arena: &mut TypeArena,
) -> Option<TypeOrPack> {
  let mut state = TraversalState::traversal_state_type_pack_id_not_null_builtin_types_type_arena(
    unsafe { follow_type_pack_id(root) },
    builtin_types,
    arena,
  );
  if traverse_path(&mut state, path) {
    if state.encountered_error_suppression {
      return Some(TypeOrPack::V0(builtin_types.error_type));
    }
    Some(state.current)
  } else {
    None
  }
}
