use crate::{
  records::refine::Refine,
  type_aliases::{
    def_id_control_flow_graph::DefId, refinement_id_control_flow_graph::RefinementId,
  },
};

impl Refine {
  pub fn new(definition: DefId, prop: RefinementId) -> Self {
    Self { definition, prop }
  }
}
