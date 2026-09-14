use crate::records::{
  type_reduction_reentrancy_guard::TypeReductionReentrancyGuard,
  unifier_shared_state::UnifierSharedState,
};

impl TypeReductionReentrancyGuard {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn type_reduction_reentrancy_guard_not_null_unifier_shared_state(
    shared_state: *mut UnifierSharedState,
  ) -> Self {
    unsafe {
      if !shared_state.is_null() {
        (*shared_state).reentrant_type_reduction = true;
      }
    }
    Self { shared_state }
  }
}
