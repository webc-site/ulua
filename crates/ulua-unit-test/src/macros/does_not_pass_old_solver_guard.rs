#[macro_export]
macro_rules! DOES_NOT_PASS_OLD_SOLVER_GUARD {
  () => {
    $crate::macros::does_not_pass_old_solver_guard_impl::DOES_NOT_PASS_OLD_SOLVER_GUARD_IMPL!();
  };
}

pub use DOES_NOT_PASS_OLD_SOLVER_GUARD;
