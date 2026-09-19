use crate::records::unifier_shared_state::UnifierSharedState;

/// cpp `TypeReductionReentrancyGuard`（UnifierSharedState.h:55-66）：构造置位、
/// 析构复位。复位必须发生在 Drop 而不是调用点末尾——本仓把 ICE 建模为 panic，
/// 手工复位会被 unwind 跳过，导致 `reentrant_type_reduction` 永久为 true，
/// 之后每次归约静默返回空结果。
#[derive(Debug)]
pub struct TypeReductionReentrancyGuard {
  pub shared_state: *mut UnifierSharedState,
}

impl Drop for TypeReductionReentrancyGuard {
  fn drop(&mut self) {
    if !self.shared_state.is_null() {
      unsafe {
        (*self.shared_state).reentrant_type_reduction = false;
      }
    }
  }
}
