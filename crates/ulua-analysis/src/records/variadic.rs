//! Source: `Analysis/include/Luau/Refinement.h`

use alloc::vec::Vec;

use crate::type_aliases::refinement_id_refinement::RefinementId;

#[derive(Debug, Clone)]
pub struct Variadic {
  pub refinements: Vec<RefinementId>,
}
