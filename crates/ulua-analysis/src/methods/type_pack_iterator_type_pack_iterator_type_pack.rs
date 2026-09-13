use core::ptr::null;

use crate::{
  records::type_pack_iterator::TypePackIterator, type_aliases::type_pack_id::TypePackId,
};
impl TypePackIterator {
  pub fn new() -> Self {
    // TypePackId is currently a stub type; null sentinel deferred until TypePackId = *const TypePackVar
    let null_tp: TypePackId = Default::default();
    Self {
      current_type_pack: null_tp,
      tail_cycle_check: null_tp,
      tp: null(),
      current_index: 0,
      log: null(),
    }
  }
}

impl Default for TypePackIterator {
  fn default() -> Self {
    Self::new()
  }
}
