#[macro_export]
macro_rules! DOES_NOT_PASS_NEW_SOLVER_GUARD {
  () => {
    $crate::macros::does_not_pass_new_solver_guard_impl::DOES_NOT_PASS_NEW_SOLVER_GUARD_IMPL!(
      ::core::line!()
    );
  };
}

pub use DOES_NOT_PASS_NEW_SOLVER_GUARD;
