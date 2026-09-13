use crate::{
  functions::{
    begin_type_pack_alt_d::begin_type_pack_id_txn_log, end_type_pack::end_type_pack_id,
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

  while it.operator_ne(&end_iter) {
    let ty = *it.operator_deref();
    if is_optional(ty) {
      optional_count += 1;
    } else {
      min_count += optional_count;
      optional_count = 0;
      min_count += 1;
    }

    it.operator_inc();
  }

  if it
    .tail()
    .map(|tail_tp| is_variadic_tail(tail_tp, unsafe { &*log }, include_hidden_variadics))
    .unwrap_or(false)
  {
    (min_count, None)
  } else {
    (min_count, Some(min_count + optional_count))
  }
}
