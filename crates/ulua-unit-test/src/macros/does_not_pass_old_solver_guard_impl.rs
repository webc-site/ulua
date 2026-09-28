#[macro_export]
macro_rules! DOES_NOT_PASS_OLD_SOLVER_GUARD_IMPL {
  () => {
    // cpp `DOES_NOT_PASS_OLD_SOLVER_GUARD_IMPL()` 的原式是
    // `ScopedFastFlag{FFlag::DebugLuauForceOldSolver, FFlag::DebugLuauForceAllOldSolverTests.get()}`。
    // 上游旗标 `DebugLuauForceAllOldSolverTests` 已按 r7 deadcode 仲裁摘除：全仓无任何路径
    // 置 true，恒 false —— 该守卫的实际语义就是「在 old-solver 语境的夹具里显式关掉 old
    // solver」，此处直接内联常量，行为逐项等价。
    let _sff = $crate::type_aliases::scoped_fast_flag::ScopedFastFlag::new(
      &ulua_common::fflag::DebugLuauForceOldSolver,
      false,
    );
  };
}

pub use DOES_NOT_PASS_OLD_SOLVER_GUARD_IMPL;
