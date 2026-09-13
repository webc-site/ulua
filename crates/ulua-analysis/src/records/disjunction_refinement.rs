//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/Refinement.h:40:disjunction`
//! Source: `Analysis/include/Luau/Refinement.h`
//! Graph edges:
//! - declared_by: source_file Analysis/include/Luau/Refinement.h
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypedAllocator.h
//!   - includes -> source_file Common/include/Luau/Variant.h
//!   - includes -> source_file Analysis/include/Luau/TypeFwd.h
//! - incoming:
//!   - declares <- source_file Analysis/include/Luau/Refinement.h
//!   - type_ref <- type_alias Refinement (Analysis/include/Luau/Refinement.h)
//! - outgoing:
//!   - type_ref -> type_alias RefinementId (Analysis/include/Luau/Refinement.h)
//!   - translates_to -> rust_item Disjunction

use crate::type_aliases::refinement_id_refinement::RefinementId;

#[derive(Debug, Clone)]
pub struct Disjunction {
  pub lhs: RefinementId,
  pub rhs: RefinementId,
}
