//! Source: `tests/Unifier2.test.cpp`

use core::ptr::NonNull;

use ulua_analysis::records::{
  builtin_types::BuiltinTypes, internal_error_reporter::InternalErrorReporter, scope::Scope,
  to_string_options::ToStringOptions, type_arena::TypeArena, unifier_2::Unifier2,
};
use ulua_common::fflag;

use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
#[derive(Debug)]
pub struct Unifier2Fixture {
  pub arena: Box<TypeArena>,
  pub builtin_types: Box<BuiltinTypes>,
  pub scope: Box<Scope>,
  pub ice_reporter: Box<InternalErrorReporter>,
  pub u2: Unifier2,
  pub opts: ToStringOptions,
  pub _sff: ScopedFastFlag,
}

impl Unifier2Fixture {
  pub fn new() -> Self {
    let mut arena = Box::new(TypeArena::default());
    let mut builtin_types = Box::new(BuiltinTypes::new());
    let mut scope = Box::new(Scope::scope_type_pack_id(builtin_types.any_type_pack()));
    let mut ice_reporter = Box::new(InternalErrorReporter::default());

    let u2 = Unifier2::unifier_2_not_null_type_arena_not_null_builtin_types_not_null_scope_not_null_internal_error_reporter(
            NonNull::from(&mut *arena),
            NonNull::from(&mut *builtin_types),
            NonNull::from(&mut *scope),
            NonNull::from(&mut *ice_reporter),
        );

    Self {
      arena,
      builtin_types,
      scope,
      ice_reporter,
      u2,
      opts: ToStringOptions::default(),
      _sff: ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
    }
  }
}

impl Default for Unifier2Fixture {
  fn default() -> Self {
    Self::new()
  }
}
