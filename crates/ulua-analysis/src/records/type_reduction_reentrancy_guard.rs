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
      // Safety: 上方判空守卫——null 是「未接线共享状态」哨兵（C++ 默认构造不持有
      // sharedState），无复位目标；非空时它是构造期接线的 UnifierSharedState
      // （TypeCheckSharedState 级持有，比 guard 长寿），此处只把一个 bool 写回
      // false，单线程下 Drop 先于共享状态销毁执行，无并发触碰。
      unsafe {
        (*self.shared_state).reentrant_type_reduction = false;
      }
    }
  }
}
