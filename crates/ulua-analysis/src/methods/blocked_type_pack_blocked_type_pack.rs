use crate::{functions::fresh_index::fresh_index, records::blocked_type_pack::BlockedTypePack};

impl BlockedTypePack {
  pub fn blocked_type_pack_blocked_type_pack(&mut self) {
    self.index = fresh_index() as usize;
  }
}
