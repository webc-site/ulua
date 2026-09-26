use crate::{
  records::{
    apply_mapped_generics::ApplyMappedGenerics, arena_handle::Handle, builtin_types::BuiltinTypes,
    internal_error_reporter::InternalErrorReporter, substitution::Substitution,
    subtyping_environment::SubtypingEnvironment, txn_log::TxnLog, type_arena::TypeArena,
  },
  type_aliases::type_id::TypeId,
};

impl SubtypingEnvironment {
  /// C++ `SubtypingEnvironment::applyMappedGenerics` (`Subtyping.cpp:504-513`):
  ///
  /// ```cpp
  /// ApplyMappedGenerics amg{builtinTypes, arena, *this, iceReporter};
  /// return amg.substitute(ty);
  /// ```
  ///
  /// `ApplyMappedGenerics` extends `Substitution` and inherits `substitute`,
  /// whose traversal virtual-dispatches into the overridden `isDirty` /
  /// `clean` / `ignoreChildren`. The Rust `ApplyMappedGenerics` now embeds
  /// `base: Substitution` and installs those overrides into the
  /// `SubstitutionVtable` from its `substitute_type_id` wrapper.
  pub fn apply_mapped_generics(
    &mut self,
    builtin_types: Handle<BuiltinTypes>,
    arena: Handle<TypeArena>,
    ty: TypeId,
    ice_reporter: *mut InternalErrorReporter,
  ) -> Option<TypeId> {
    let mut amg = ApplyMappedGenerics {
      base: Substitution::substitution_new(TxnLog::empty(), Some(arena)),
      builtin_types,
      arena,
      ice_reporter,
      env: self as *mut SubtypingEnvironment,
    };
    amg.substitute_type_id(ty)
  }
}
