use crate::records::{generic_pack_mapping::GenericPackMapping, traversal_state::TraversalState};

impl TraversalState {
  pub fn traverse_type_path_generic_pack_mapping(&mut self, mapping: GenericPackMapping) -> bool {
    if self.check_invariants() {
      return false;
    }
    self.update_current_type_pack_id(mapping.mapped_type);
    true
  }
}
