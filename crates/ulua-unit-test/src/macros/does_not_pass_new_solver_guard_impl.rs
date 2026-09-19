#[macro_export]
macro_rules! DOES_NOT_PASS_NEW_SOLVER_GUARD_IMPL {
  ($line:expr) => {
    let _sff = $crate::type_aliases::scoped_fast_flag::ScopedFastFlag::new(
      &ulua_common::fflag::DebugLuauForceOldSolver,
      !ulua_common::fflag::DebugLuauForceAllNewSolverTests.get(),
    );
  };
}

pub use DOES_NOT_PASS_NEW_SOLVER_GUARD_IMPL;
