use crate::records::{
  arena_handle::alias, type_reduction_reentrancy_guard::TypeReductionReentrancyGuard,
  unifier_shared_state::UnifierSharedState,
};

impl TypeReductionReentrancyGuard {
  /// 前置契约（本函数体经 safe 门面完成指针借用，无 unsafe 操作；以下为文档约定）
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub(crate) fn type_reduction_reentrancy_guard_not_null_unifier_shared_state(
    shared_state: *mut UnifierSharedState,
  ) -> Self {
    if !shared_state.is_null() {
      alias(shared_state).reentrant_type_reduction = true;
    }
    Self { shared_state }
  }
}
