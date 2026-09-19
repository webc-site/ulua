use crate::{
  records::remove_dead_store_state::RemoveDeadStoreState, traits::tag_access::TagAccess,
};

/// cpp OptimizeDeadStore.cpp：get/set 直接读写 `state.info[i].knownTag`
impl TagAccess for RemoveDeadStoreState {
  fn get_tag(&self, i: usize) -> u8 {
    self.info[i].known_tag
  }

  fn set_tag(&mut self, i: usize, tag: u8) {
    self.info[i].known_tag = tag;
  }
}
