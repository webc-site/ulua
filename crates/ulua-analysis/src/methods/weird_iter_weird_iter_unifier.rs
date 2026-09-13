use crate::{
  records::{txn_log::TxnLog, type_pack::TypePack, weird_iter::WeirdIter},
  type_aliases::type_pack_id::TypePackId,
};

impl WeirdIter {
  pub fn weird_iter_type_pack_id_txn_log(&mut self, mut pack_id: TypePackId, log: &mut TxnLog) {
    self.pack_id = pack_id;
    self.log = log as *mut TxnLog;
    self.pack = log.txn_log_get_mutable::<TypePack, TypePackId>(pack_id);
    self.index = 0;
    self.growing = false;
    while !self.pack.is_null()
      && unsafe { (*self.pack).head.is_empty() }
      && unsafe { (*self.pack).tail.is_some() }
    {
      pack_id = unsafe { (*self.pack).tail.unwrap() };
      self.pack = log.txn_log_get_mutable::<TypePack, TypePackId>(pack_id);
    }
  }
}
