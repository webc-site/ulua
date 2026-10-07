//! Source: `Analysis/include/Luau/Refinement.h`

use crate::type_aliases::refinement_id_refinement::RefinementId;

#[derive(Debug, Clone)]
pub struct Disjunction {
  pub lhs: RefinementId,
  pub rhs: RefinementId,
}
