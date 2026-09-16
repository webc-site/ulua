use crate::records::{
  builtin_types::BuiltinTypes, intersection_builder::IntersectionBuilder, type_arena::TypeArena,
  type_ids::TypeIds,
};

impl IntersectionBuilder {
  pub fn new(arena: *mut TypeArena, builtin_types: *mut BuiltinTypes) -> Self {
    Self {
      arena,
      builtin_types,
      parts: TypeIds::new(),
      is_bottom: false,
    }
  }
}
