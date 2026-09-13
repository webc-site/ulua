#[macro_export]
macro_rules! DOES_NOT_PASS_OLD_SOLVER_GUARD_IMPL {
  ($line:expr) => {
    let _sff = $crate::type_aliases::scoped_fast_flag::ScopedFastFlag::new(
      &ulua_common::FFlag::DebugLuauForceOldSolver,
      ulua_common::FFlag::DebugLuauForceAllOldSolverTests.get(),
    );
  };
}

pub use DOES_NOT_PASS_OLD_SOLVER_GUARD_IMPL;
