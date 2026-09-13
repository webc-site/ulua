use alloc::vec::Vec;

use crate::{
  records::{
    builtin_types::BuiltinTypes, replace_generics::ReplaceGenerics, scope::Scope, txn_log::TxnLog,
    type_arena::TypeArena, type_level::TypeLevel,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl ReplaceGenerics {
  pub fn reset_state(
    &mut self,
    log: *const TxnLog,
    arena: *mut TypeArena,
    builtin_types: *mut BuiltinTypes,
    level: TypeLevel,
    scope: *mut Scope,
    generics: Vec<TypeId>,
    generic_packs: Vec<TypePackId>,
  ) {
    self.base.reset_state(log, arena);

    self.builtin_types = builtin_types;

    self.level = level;
    self.scope = scope;

    self.generics = generics;
    self.generic_packs = generic_packs;
  }
}
