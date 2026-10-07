#[macro_export]
macro_rules! DOES_NOT_PASS_NEW_SOLVER_GUARD_IMPL {
  () => {
    // cpp `DOES_NOT_PASS_NEW_SOLVER_GUARD_IMPL()` 的原式是
    // `ScopedFastFlag{FFlag::DebugLuauForceOldSolver, !FFlag::DebugLuauForceAllNewSolverTests.get()}`。
    // 上游旗标 `DebugLuauForceAllNewSolverTests` 已按 r7 deadcode 仲裁摘除：全仓（含上游
    // tests）无任何路径置 true，故 `!false` 恒为 `true` —— 该守卫的实际语义就是「把本用例
    // 钉在 old solver 上跑」，此处直接内联常量，行为逐项等价。
    let _sff = $crate::type_aliases::scoped_fast_flag::ScopedFastFlag::new(
      &ulua_common::fflag::DebugLuauForceOldSolver,
      true,
    );
  };
}

pub use DOES_NOT_PASS_NEW_SOLVER_GUARD_IMPL;
