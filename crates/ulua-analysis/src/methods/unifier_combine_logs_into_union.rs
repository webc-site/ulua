use crate::records::{txn_log::TxnLog, type_arena::TypeArena, unifier::Unifier};

impl Unifier {
  /// # Safety
  /// 调用方须保证 `arena` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn unifier_combine_logs_into_union(
    &mut self,
    logs: Vec<TxnLog>,
    arena: *mut TypeArena,
  ) -> TxnLog {
    let mut result = TxnLog::new();
    for log in logs {
      unsafe { result.concat_as_union(log, arena) };
    }
    result
  }
}
