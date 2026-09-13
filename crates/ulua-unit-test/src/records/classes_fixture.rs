//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Record:Luau.UnitTest:tests/TypeInfer.classes.test.cpp:22:classes_fixture`
//! Source: `tests/TypeInfer.classes.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.classes.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.classes.test.cpp
//!   - type_ref <- method ClassesFixture::getFrontend (tests/TypeInfer.classes.test.cpp)
//! - outgoing:
//!   - type_ref -> record Fixture (tests/Fixture.h)
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - translates_to -> rust_item ClassesFixture

use ulua_common::FFlag;

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
        &FFlag::DebugLuauUserDefinedClasses,
        true,
      ),
      sff_luau_allow_global_declaration_to_be_called_class: ScopedFastFlag::new(
        &FFlag::LuauAllowGlobalDeclarationToBeCalledClass,
        true,
      ),
      old_solver_guard: ScopedFastFlag::new(
        &FFlag::DebugLuauForceOldSolver,
        FFlag::DebugLuauForceAllOldSolverTests.get(),
      ),
    }
  }
}
