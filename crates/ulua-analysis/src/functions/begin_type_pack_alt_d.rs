use crate::{
  records::{txn_log::TxnLog, type_pack_iterator::TypePackIterator},
  type_aliases::type_pack_id::TypePackId,
};

pub(crate) fn begin(tp: TypePackId, log: *const TxnLog) -> TypePackIterator {
  let mut it = TypePackIterator::new();
  unsafe { it.type_pack_iterator_type_pack_id_txn_log(tp, log) };
  it
}

pub(crate) use crate::functions::begin_type_pack_alt_d::begin as begin_type_pack_id_txn_log;
