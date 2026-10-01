use crate::records::{
  arena_handle::alias, type_reduction_reentrancy_guard::TypeReductionReentrancyGuard,
  unifier_shared_state::UnifierSharedState,
};

impl TypeReductionReentrancyGuard {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub(crate) unsafe fn type_reduction_reentrancy_guard_not_null_unifier_shared_state(
    shared_state: *mut UnifierSharedState,
  ) -> Self {
    if !shared_state.is_null() {
      alias(shared_state).reentrant_type_reduction = true;
    }
    Self { shared_state }
  }
}
