use crate::{
  functions::is_variadic_type_pack_alt_b::is_variadic_type_pack_id_txn_log,
  records::txn_log::TxnLog, type_aliases::type_pack_id::TypePackId,
};

pub fn is_variadic(tp: TypePackId) -> bool {
  is_variadic_type_pack_id_txn_log(tp, unsafe { &*TxnLog::empty() })
}
