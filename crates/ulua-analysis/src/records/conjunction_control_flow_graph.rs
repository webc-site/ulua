use crate::type_aliases::refinement_id_control_flow_graph::RefinementId;

#[derive(Debug, Clone)]
pub struct Conjunction {
  pub lhs: RefinementId,
  pub rhs: RefinementId,
}
