use alloc::string::String;
use core::ffi::c_void;

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
      def: def.as_ptr() as *const c_void,
      prop_name: Some(String::from(prop_name)),
    })
  }
}
