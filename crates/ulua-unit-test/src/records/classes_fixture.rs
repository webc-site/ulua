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
      // cpp `TypeInfer.classes.test.cpp` 的 ClassesFixture 用
      // `DOES_NOT_PASS_OLD_SOLVER_GUARD()`（展开即 `Fixture.h:42` 的
      // `ScopedFastFlag{FFlag::DebugLuauForceOldSolver, FFlag::DebugLuauForceAllOldSolverTests}`）；
      // 该上游旗标已按 r7 deadcode 仲裁摘除（全仓无任何路径置 true，恒 false），故此处内联
      // `false`：夹具始终显式关闭 old solver 强制，行为逐项等价。
      old_solver_guard: ScopedFastFlag::new(&fflag::DebugLuauForceOldSolver, false),
    }
  }
}
