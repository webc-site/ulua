use crate::{
  functions::{follow_type, follow_type_pack},
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, traversal_state::TraversalState,
    type_arena::TypeArena,
  },
  type_aliases::{type_id::TypeId, type_or_pack::TypeOrPack, type_pack_id::TypePackId},
};

impl TraversalState {
  pub fn traversal_state_type_id_not_null_builtin_types_type_arena(
    root: TypeId,
    builtin_types: &BuiltinTypes,
    arena: &mut TypeArena,
  ) -> Self {
    TraversalState {
      current: TypeOrPack::V0(follow_type::follow(root)),
      builtin_types,
      arena: Handle::from_ptr(arena),
      steps: 0,
      encountered_error_suppression: false,
    }
  }

  pub(crate) fn traversal_state_type_pack_id_not_null_builtin_types_type_arena(
    root: TypePackId,
    builtin_types: &BuiltinTypes,
    arena: &mut TypeArena,
  ) -> Self {
    TraversalState {
      current: TypeOrPack::V1(follow_type_pack::follow(root)),
      builtin_types,
      arena: Handle::from_ptr(arena),
      steps: 0,
      encountered_error_suppression: false,
    }
  }
}
