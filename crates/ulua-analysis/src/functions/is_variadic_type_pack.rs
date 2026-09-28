use crate::{
  functions::{flatten_type_pack::flatten, is_variadic_tail::is_variadic_tail},
  records::txn_log::TxnLog,
  type_aliases::type_pack_id::TypePackId,
};

pub fn is_variadic(tp: TypePackId) -> bool {
  is_variadic_txn_log(tp, unsafe { &*TxnLog::empty() })
}

pub fn is_variadic_txn_log(tp: TypePackId, log: &TxnLog) -> bool {
  let (_, tail) = flatten(tp, log);

  if let Some(tail_tp) = tail {
    return is_variadic_tail(tail_tp, log, false);
  }

  false
}
