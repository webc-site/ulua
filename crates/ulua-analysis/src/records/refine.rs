use crate::type_aliases::{
  def_id_control_flow_graph::DefId, refinement_control_flow_graph::Refinement,
};

#[derive(Debug, Clone)]
pub struct Refine {
  pub definition: DefId,
  pub prop: *const Refinement,
}
