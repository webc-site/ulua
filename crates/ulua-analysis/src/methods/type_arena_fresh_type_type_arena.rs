use crate::{
  records::{
    builtin_types::BuiltinTypes, free_type::FreeType, type_arena::TypeArena, type_level::TypeLevel,
  },
  type_aliases::type_id::TypeId,
};

impl TypeArena {
  pub fn fresh_type_not_null_builtin_types_type_level(
    &mut self,
    builtins: &BuiltinTypes,
    level: TypeLevel,
  ) -> TypeId {
    self.add_type(FreeType {
      level,
      lower_bound: builtins.never_type,
      upper_bound: builtins.unknown_type,
      ..FreeType::default()
    })
  }
}
