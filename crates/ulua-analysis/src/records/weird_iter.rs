use crate::{
  records::{scope::Scope, txn_log::TxnLog, type_level::TypeLevel, type_pack::TypePack},
  type_aliases::type_pack_id::TypePackId,
};
#[derive(Debug, Clone)]
pub struct WeirdIter {
  pub(crate) pack_id: TypePackId,
  pub(crate) log: *mut TxnLog,
  pub(crate) pack: *mut TypePack,
  pub(crate) index: usize,
  pub(crate) growing: bool,
  pub(crate) level: TypeLevel,
  pub(crate) scope: *mut Scope,
}
