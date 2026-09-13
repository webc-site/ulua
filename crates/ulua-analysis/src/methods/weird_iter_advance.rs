use crate::records::{type_pack::TypePack, weird_iter::WeirdIter};

impl WeirdIter {
  pub fn weird_iter_advance(&mut self) -> bool {
    if self.pack.is_null() {
      return self.weird_iter_good();
    }
    if self.index < unsafe { (*self.pack).head.len() } {
      self.index += 1;
    }
    if self.growing || self.index < unsafe { (*self.pack).head.len() } {
      return self.weird_iter_good();
    }
    if let Some(tail) = unsafe { (*self.pack).tail } {
      self.pack_id = unsafe { (*self.log).follow_type_pack_id(tail) };
      self.pack = unsafe { (*self.log).txn_log_get_mutable::<TypePack, _>(self.pack_id) };
      self.index = 0;
    }
    self.weird_iter_good()
  }
}
