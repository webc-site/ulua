use crate::{
  methods::{
    non_strict_context_conjunction::non_strict_context_conjunction,
    non_strict_context_disjunction::non_strict_context_disjunction,
  },
  records::{
    builtin_types::BuiltinTypes, def::Def, non_strict_context::NonStrictContext,
    type_arena::TypeArena,
  },
  type_aliases::{def_id_def::DefId, type_id::TypeId},
};
impl NonStrictContext {
  pub fn find_def_id(&self, def: &DefId) -> Option<TypeId> {
    let d: *const Def = *def;
    self.find_def(d)
  }

  pub fn disjunction(
    builtin_types: *mut BuiltinTypes,
    arena: *mut TypeArena,
    left: &NonStrictContext,
    right: &NonStrictContext,
  ) -> NonStrictContext {
    non_strict_context_disjunction(builtin_types, arena, left, right)
  }

  pub fn conjunction(
    builtin_types: *mut BuiltinTypes,
    arena: *mut TypeArena,
    left: &NonStrictContext,
    right: &NonStrictContext,
  ) -> NonStrictContext {
    non_strict_context_conjunction(builtin_types, arena, left, right)
  }
}
