use crate::{
  records::{txn_log::TxnLog, type_pack::TypePack},
  type_aliases::type_pack_id::TypePackId,
};

#[derive(Debug, Clone)]
pub struct TypePackIterator {
  pub(crate) current_type_pack: TypePackId,
  pub(crate) tail_cycle_check: TypePackId,
  pub(crate) tp: *const TypePack,
  pub(crate) current_index: usize,
  pub(crate) log: *const TxnLog,
}
