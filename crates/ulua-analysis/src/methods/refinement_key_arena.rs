//! `refinement_key_arena` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::string::String;
use core::ptr::null;

use crate::{
  records::{refinement_key::RefinementKey, refinement_key_arena::RefinementKeyArena},
  type_aliases::def_id_refinement::DefId,
};

impl RefinementKeyArena {
  pub fn empty(&self) -> bool {
    self.allocator.empty()
  }
}

impl RefinementKeyArena {
  pub fn leaf(&mut self, def: DefId) -> *const RefinementKey {
    self.allocator.allocate(RefinementKey {
      parent: null(),
      def,
      prop_name: None,
    })
  }
}

impl RefinementKeyArena {
  pub fn node(
    &mut self,
    parent: *const RefinementKey,
    def: DefId,
    prop_name: &str,
  ) -> *const RefinementKey {
    self.allocator.allocate(RefinementKey {
      parent,
      def,
      prop_name: Some(String::from(prop_name)),
    })
  }
}
