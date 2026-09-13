use crate::{
  records::{
    builtin_types::BuiltinTypes, free_type::FreeType, scope::Scope, type_arena::TypeArena,
  },
  type_aliases::type_id::TypeId,
};

impl TypeArena {
  pub fn fresh_type_not_null_builtin_types_scope(
    &mut self,
    builtins: &BuiltinTypes,
    scope: *mut Scope,
  ) -> TypeId {
    self.add_type(FreeType {
      scope,
      lower_bound: builtins.never_type,
      upper_bound: builtins.unknown_type,
      ..FreeType::default()
    })
  }
}
