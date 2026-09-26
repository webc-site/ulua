use crate::{
  enums::polarity::Polarity,
  records::{
    builtin_types::BuiltinTypes, free_type::FreeType, scope::Scope, type_arena::TypeArena,
  },
  type_aliases::type_id::TypeId,
};

pub fn fresh_type(
  arena: &mut TypeArena,
  builtin_types: &BuiltinTypes,
  scope: *mut Scope,
  polarity: Polarity,
) -> TypeId {
  let free_type = FreeType::free_type_scope_type_id_type_id_polarity(
    scope,
    builtin_types.never_type,
    builtin_types.unknown_type,
    polarity,
  );
  arena.add_type(free_type)
}
