use crate::{
  functions::follow_type::follow_type_id,
  records::{builtin_types::BuiltinTypes, traversal_state::TraversalState, type_arena::TypeArena},
  type_aliases::{type_id::TypeId, type_or_pack::TypeOrPack},
};
impl TraversalState {
  pub fn traversal_state_type_id_not_null_builtin_types_type_arena(
    root: TypeId,
    builtin_types: &BuiltinTypes,
    arena: &mut TypeArena,
  ) -> Self {
    TraversalState {
      current: TypeOrPack::V0(follow_type_id(root)),
      builtin_types,
      arena,
      steps: 0,
      encountered_error_suppression: false,
    }
  }
}
