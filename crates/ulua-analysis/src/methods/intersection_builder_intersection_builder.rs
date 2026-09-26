use crate::records::{
  arena_handle::Handle, builtin_types::BuiltinTypes, intersection_builder::IntersectionBuilder,
  type_arena::TypeArena, type_ids::TypeIds,
};

impl IntersectionBuilder {
  pub fn new(arena: Handle<TypeArena>, builtin_types: Handle<BuiltinTypes>) -> Self {
    Self {
      arena,
      builtin_types,
      parts: TypeIds::new(),
      is_bottom: false,
    }
  }
}
