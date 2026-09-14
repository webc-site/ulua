use crate::records::{
  builtin_types::BuiltinTypes, type_arena::TypeArena, type_ids::TypeIds,
  union_builder::UnionBuilder,
};
impl UnionBuilder {
  pub fn new(arena: *mut TypeArena, builtin_types: *mut BuiltinTypes) -> Self {
    Self {
      arena,
      builtin_types,
      options: TypeIds::new(),
      is_top: false,
    }
  }
}
