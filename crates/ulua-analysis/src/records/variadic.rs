//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/Refinement.h:24:variadic`
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
//!   - type_ref <- method ConstraintGenerator::computeRefinement (Analysis/src/ConstraintGenerator.cpp)
//!   - type_ref <- method RefinementArena::variadic (Analysis/src/Refinement.cpp)
//!   - type_ref <- method Subtyping::isTailCovariantWithTail (Analysis/src/Subtyping.cpp)
//!   - type_ref <- method Subtyping::isCovariantWith (Analysis/src/Subtyping.cpp)
//!   - type_ref <- method PathBuilder::variadic (Analysis/src/TypePath.cpp)
//!   - type_ref <- method TraversalState::traverse (Analysis/src/TypePath.cpp)
//!   - type_ref <- function to_string (Analysis/src/TypePath.cpp)
//!   - type_ref <- function toStringHuman (Analysis/src/TypePath.cpp)
//!   - type_ref <- test type_infer_aliases_mismatched_generic_pack_type_param (tests/TypeInfer.aliases.test.cpp)
//!   - type_ref <- test type_path_variadic (tests/TypePath.test.cpp)
//!   - type_ref <- test type_path_fields (tests/TypePath.test.cpp)
//! - outgoing:
//!   - type_ref -> type_alias RefinementId (Analysis/include/Luau/Refinement.h)
//!   - translates_to -> rust_item Variadic

use alloc::vec::Vec;

use crate::type_aliases::refinement_id_refinement::RefinementId;

#[derive(Debug, Clone)]
pub struct Variadic {
  pub refinements: Vec<RefinementId>,
}
