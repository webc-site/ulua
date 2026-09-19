//! Source: `Analysis/include/Luau/Refinement.h`

// Refinement.h:22 — using RefinementId = Refinement*; (can be null)
use crate::type_aliases::refinement_refinement::Refinement;
pub type RefinementId = *mut Refinement;
