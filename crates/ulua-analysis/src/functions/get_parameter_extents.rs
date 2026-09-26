use crate::{
  functions::{
    begin_type_pack::begin_type_pack_id_txn_log, end_type_pack::end_type_pack_id,
    is_optional::is_optional, is_variadic_tail::is_variadic_tail,
  },
  records::txn_log::TxnLog,
  type_aliases::type_pack_id::TypePackId,
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn get_parameter_extents(
  log: *const TxnLog,
  tp: TypePackId,
  include_hidden_variadics: bool,
) -> (usize, Option<usize>) {
  let mut min_count = 0usize;
  let mut optional_count = 0usize;

  let mut it = begin_type_pack_id_txn_log(tp, log);
  let end_iter = end_type_pack_id(tp);

  while it != end_iter {
    let ty = *it.current();
    if is_optional(ty) {
      optional_count += 1;
    } else {
      min_count += optional_count;
      optional_count = 0;
      min_count += 1;
    }

    it.advance();
  }

  if it
    .tail()
    // Safety: `log` 按 C++ 原契约始终指向存活的 TxnLog（非空、对齐、地址稳定），上方
    // `begin_type_pack_id_txn_log(tp, log)` 亦以同一 log 裸指针工作；此处 `&*log` 仅重建一个
    // 生命周期短到本次 `is_variadic_tail` 调用的只读借用，单线程遍历无并发可变借用冲突。
    .map(|tail_tp| is_variadic_tail(tail_tp, unsafe { &*log }, include_hidden_variadics))
    .unwrap_or(false)
  {
    (min_count, None)
  } else {
    (min_count, Some(min_count + optional_count))
  }
}
