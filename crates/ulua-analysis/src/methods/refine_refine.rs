use crate::{
  records::refine::Refine,
  type_aliases::{def_id_control_flow_graph::DefId, refinement_control_flow_graph::Refinement},
};

impl Refine {
  pub fn new(definition: DefId, prop: *const Refinement) -> Self {
    Self { definition, prop }
  }
}
