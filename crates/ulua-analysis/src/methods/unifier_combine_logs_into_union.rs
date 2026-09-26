use crate::records::{
  arena_handle::Handle, txn_log::TxnLog, type_arena::TypeArena, unifier::Unifier,
};

impl Unifier {
  /// `arena` 为 `Handle` 句柄，其目标存活与独占别名契约见 `arena_handle` 模块头。
  pub(crate) fn unifier_combine_logs_into_union(
    &mut self,
    logs: Vec<TxnLog>,
    arena: Handle<TypeArena>,
  ) -> TxnLog {
    let mut result = TxnLog::new();
    for log in logs {
      // Safety: `arena` 沿用本方法文档的句柄契约（目标存活、借用期内无并存可变
      // 别名），与 concat_as_union 的 # Safety 前置条件一致。
      unsafe { result.concat_as_union(log, arena) };
    }
    result
  }
}
