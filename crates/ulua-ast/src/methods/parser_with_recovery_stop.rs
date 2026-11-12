//! 错误恢复停点计数的作用域守卫：进入上下文时对指定 token 计数 +1，离开时 -1。
//!
//! cpp `matchRecoveryStopOnToken[token] += 1; ... -= 1;` 的 RAII 化：作用域结束（含
//! 提前返回）自动减计数，避免手工配对遗漏导致恢复行为漂移。

use crate::records::parser::Parser;

/// `match_recovery_stop_on_token[idx]` 的作用域守卫。
///
/// 持有 `*mut Parser` 以在 `Drop` 时回写计数；构造时已完成 `+= 1`。
/// 安全性前提：`Parser` 在守卫存活期内保持驻留（解析方法栈上 `&mut self` 满足），
/// 且 `Drop` 执行时不存在对同一计数槽位的并发别名可变访问。
pub(crate) struct RecoveryStopGuard {
  parser: *mut Parser,
  idx: usize,
}

impl Drop for RecoveryStopGuard {
  #[inline]
  fn drop(&mut self) {
    // Safety: 守卫由 `Parser::push_recovery_stop` 构造，`parser` 指向存活的
    // Parser；`Drop` 在作用域末尾执行，此时无其他对该计数表项的活动借用。
    unsafe {
      (*self.parser).match_recovery_stop_on_token[self.idx] -= 1;
    }
  }
}

impl Parser {
  /// 提升 `match_recovery_stop_on_token[idx]` 并返回守卫；守卫 drop 时自动恢复。
  #[inline]
  #[must_use = "恢复停点守卫必须绑定到变量，否则立即 drop 导致计数失配"]
  pub(crate) fn push_recovery_stop(&mut self, idx: usize) -> RecoveryStopGuard {
    self.match_recovery_stop_on_token[idx] += 1;
    RecoveryStopGuard {
      parser: self as *mut Parser,
      idx,
    }
  }
}
