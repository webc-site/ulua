#[macro_export]
macro_rules! DOES_NOT_PASS_OLD_SOLVER_GUARD_IMPL {
  () => {
    let _sff = $crate::type_aliases::scoped_fast_flag::ScopedFastFlag::new(
      &ulua_common::fflag::DebugLuauForceOldSolver,
      ulua_common::fflag::DebugLuauForceAllOldSolverTests.get(),
    );
  };
}

pub use DOES_NOT_PASS_OLD_SOLVER_GUARD_IMPL;
