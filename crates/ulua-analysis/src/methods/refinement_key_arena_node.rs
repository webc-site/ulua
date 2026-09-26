use alloc::string::String;

use crate::{
  records::{refinement_key::RefinementKey, refinement_key_arena::RefinementKeyArena},
  type_aliases::def_id_refinement::DefId,
};
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
