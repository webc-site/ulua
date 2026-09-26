use crate::records::{
  arena_handle::Handle, builtin_types::BuiltinTypes, type_arena::TypeArena, type_ids::TypeIds,
  union_builder::UnionBuilder,
};
impl UnionBuilder {
  pub fn new(arena: Handle<TypeArena>, builtin_types: Handle<BuiltinTypes>) -> Self {
    Self {
      arena,
      builtin_types,
      options: TypeIds::new(),
      is_top: false,
    }
  }
}
