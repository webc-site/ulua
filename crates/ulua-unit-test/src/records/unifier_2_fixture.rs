//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Record:Luau.UnitTest:tests/Unifier2.test.cpp:18:unifier_2_fixture`
//! Source: `tests/Unifier2.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Unifier2.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/TypeArena.h
//!   - includes -> source_file Analysis/include/Luau/Unifier2.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Unifier2.test.cpp
//!   - type_ref <- method Unifier2Fixture::freshType (tests/Unifier2.test.cpp)
//!   - type_ref <- method Unifier2Fixture::to_string (tests/Unifier2.test.cpp)
//!   - type_ref <- method Unifier2Fixture::to_string (tests/Unifier2.test.cpp)
//! - outgoing:
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> record BuiltinTypes (Analysis/include/Luau/Type.h)
//!   - type_ref -> record InternalErrorReporter (Analysis/include/Luau/Error.h)
//!   - type_ref -> record Unifier2 (Analysis/include/Luau/Unifier2.h)
//!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - translates_to -> rust_item Unifier2Fixture

use core::ptr::NonNull;

use ulua_analysis::records::{
  builtin_types::BuiltinTypes, internal_error_reporter::InternalErrorReporter, scope::Scope,
  to_string_options::ToStringOptions, type_arena::TypeArena, unifier_2::Unifier2,
};
use ulua_common::FFlag;

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
      _sff: ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
    }
  }
}

impl Default for Unifier2Fixture {
  fn default() -> Self {
    Self::new()
  }
}
