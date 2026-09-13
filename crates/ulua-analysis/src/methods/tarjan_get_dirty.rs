use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::tarjan::Tarjan;

impl Tarjan {
  pub fn get_dirty(&self, index: i32) -> bool {
    let index_usize = index as usize;
    LUAU_ASSERT!(index_usize < self.nodes.len());
    self.nodes[index_usize].dirty
  }
}
