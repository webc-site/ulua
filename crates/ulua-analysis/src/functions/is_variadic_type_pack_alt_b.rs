use crate::{
  functions::{flatten_type_pack_alt_b::flatten, is_variadic_tail::is_variadic_tail},
  records::txn_log::TxnLog,
  type_aliases::type_pack_id::TypePackId,
};

pub fn is_variadic(tp: TypePackId, log: &TxnLog) -> bool {
  let (_, tail) = flatten(tp, log);

  if let Some(tail_tp) = tail {
    return is_variadic_tail(tail_tp, log, false);
  }

  false
}

pub use is_variadic as is_variadic_type_pack_id_txn_log;
