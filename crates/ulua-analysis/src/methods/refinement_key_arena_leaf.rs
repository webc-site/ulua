use core::ptr::null;

use crate::{
  records::{refinement_key::RefinementKey, refinement_key_arena::RefinementKeyArena},
  type_aliases::def_id_refinement::DefId,
};
impl RefinementKeyArena {
  pub fn leaf(&mut self, def: DefId) -> *const RefinementKey {
    self.allocator.allocate(RefinementKey {
      parent: null(),
      def,
      prop_name: None,
    })
  }
}
