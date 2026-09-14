//! Generated skeleton item.
//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/Refinement.h:22:refinement_id`
//! Source: `Analysis/include/Luau/Refinement.h`
//! Graph edges:
//! - declared_by: source_file Analysis/include/Luau/Refinement.h
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypedAllocator.h
//!   - includes -> source_file Common/include/Luau/Variant.h
//!   - includes -> source_file Analysis/include/Luau/TypeFwd.h
//! - incoming:
//!   - declares <- source_file Analysis/include/Luau/Refinement.h
//!   - type_ref <- type_alias RefinementId (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref <- record Variadic (Analysis/include/Luau/Refinement.h)
//!   - type_ref <- record Negation (Analysis/include/Luau/Refinement.h)
//!   - type_ref <- record Conjunction (Analysis/include/Luau/Refinement.h)
//!   - type_ref <- record Disjunction (Analysis/include/Luau/Refinement.h)
//!   - type_ref <- record Equivalence (Analysis/include/Luau/Refinement.h)
//!   - type_ref <- function get (Analysis/include/Luau/Refinement.h)
//!   - type_ref <- record RefinementArena (Analysis/include/Luau/Refinement.h)
//! - outgoing:
//!   - type_ref -> type_alias RefinementId (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> type_alias Refinement (Analysis/include/Luau/Refinement.h)
//!   - translates_to -> rust_item RefinementId

// Refinement.h:22 — using RefinementId = Refinement*; (can be null)
use crate::type_aliases::refinement_refinement::Refinement;
pub type RefinementId = *mut Refinement;
