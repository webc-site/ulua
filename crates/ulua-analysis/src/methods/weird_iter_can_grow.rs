use crate::records::{free_type_pack::FreeTypePack, weird_iter::WeirdIter};

impl WeirdIter {
  pub fn weird_iter_can_grow(&self) -> bool {
    !unsafe { (*self.log).txn_log_get::<FreeTypePack, _>(self.pack_id) }.is_null()
  }
}
