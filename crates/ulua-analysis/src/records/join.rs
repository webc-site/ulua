use alloc::vec::Vec;

use crate::type_aliases::def_id_control_flow_graph::DefId;
#[derive(Debug, Clone)]
pub struct Join {
  pub definition: DefId,
  pub operands: Vec<DefId>,
}
