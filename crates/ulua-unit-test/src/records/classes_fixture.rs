//! Source: `tests/TypeInfer.classes.test.cpp`

use ulua_common::fflag;

use crate::{records::fixture::Fixture, type_aliases::scoped_fast_flag::ScopedFastFlag};

#[derive(Debug)]
pub struct ClassesFixture {
  pub base: Fixture,
  pub sff_debug_luau_user_defined_classes: ScopedFastFlag,
  pub sff_luau_allow_global_declaration_to_be_called_class: ScopedFastFlag,
  pub old_solver_guard: ScopedFastFlag,
}

impl Default for ClassesFixture {
  fn default() -> Self {
    Self {
      base: Fixture::fixture_bool(false),
      sff_debug_luau_user_defined_classes: ScopedFastFlag::new(
        &fflag::DebugLuauUserDefinedClasses,
        true,
      ),
      sff_luau_allow_global_declaration_to_be_called_class: ScopedFastFlag::new(
        &fflag::LuauAllowGlobalDeclarationToBeCalledClass,
        true,
      ),
      old_solver_guard: ScopedFastFlag::new(
        &fflag::DebugLuauForceOldSolver,
        fflag::DebugLuauForceAllOldSolverTests.get(),
      ),
    }
  }
}
