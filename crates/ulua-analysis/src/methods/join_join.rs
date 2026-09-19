use alloc::vec::Vec;

use crate::{records::join::Join, type_aliases::def_id_control_flow_graph::DefId};
impl Join {
  pub fn new(definition: DefId) -> Self {
    Self {
      definition,
      operands: Vec::new(),
    }
  }
}
