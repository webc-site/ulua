use core::ptr::null;

use crate::{records::refinement_key::RefinementKey, type_aliases::def_id_def::DefId};

#[derive(Debug, Clone, Copy)]
pub struct DataFlowResult {
  pub def: DefId,
  pub parent: *const RefinementKey,
}

impl Default for DataFlowResult {
  fn default() -> Self {
    Self {
      def: DefId::NULL,
      parent: null(),
    }
  }
}
