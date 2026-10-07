use crate::{
  functions::{
    begin_type_pack::begin_type_pack_id_txn_log, end_type_pack::end_type_pack_id,
    is_optional::is_optional, is_variadic_tail::is_variadic_tail,
  },
  records::{arena_handle::alias_ref, txn_log::TxnLog},
  type_aliases::type_pack_id::TypePackId,
};

/// `log` 为会话期存活 `TxnLog` 句柄；解引用收口 `alias_ref`，业务侧 safe 调用。
pub fn get_parameter_extents(
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
    // `log` 指向会话存活 TxnLog；alias_ref 重建短生命周期只读借用。
    .map(|tail_tp| is_variadic_tail(tail_tp, alias_ref(log), include_hidden_variadics))
    .unwrap_or(false)
  {
    (min_count, None)
  } else {
    (min_count, Some(min_count + optional_count))
  }
}
