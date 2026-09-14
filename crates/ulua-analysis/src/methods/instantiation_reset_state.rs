use alloc::vec::Vec;

use crate::records::{
  builtin_types::BuiltinTypes, instantiation::Instantiation, scope::Scope,
  substitution::Substitution, txn_log::TxnLog, type_arena::TypeArena, type_level::TypeLevel,
};
impl Instantiation {
  pub fn reset_state(
    &mut self,
    log: *const TxnLog,
    arena: *mut TypeArena,
    builtin_types: *mut BuiltinTypes,
    level: TypeLevel,
    scope: *mut Scope,
  ) {
    Substitution::reset_state(&mut self.base, log, arena);

    self.builtin_types = builtin_types;
    self.level = level;
    self.scope = scope;

    self.reusable_replace_generics.reset_state(
      log,
      arena,
      builtin_types,
      level,
      scope,
      Vec::new(),
      Vec::new(),
    );
  }
}
