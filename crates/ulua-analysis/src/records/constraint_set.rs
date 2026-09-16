//! Generated skeleton item.
//! Node: `cxx:Record:Luau.Analysis:Analysis/include/Luau/ConstraintSet.h:14:constraint_set`
//! Source: `Analysis/include/Luau/ConstraintSet.h`
//! Graph edges:
//! - declared_by: source_file Analysis/include/Luau/ConstraintSet.h
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Constraint.h
//!   - includes -> source_file Analysis/include/Luau/TypeIds.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//! - incoming:
//!   - declares <- source_file Analysis/include/Luau/ConstraintSet.h
//!   - type_ref <- record ConstraintGenerator (Analysis/include/Luau/ConstraintGenerator.h)
//!   - type_ref <- record ConstraintSolver (Analysis/include/Luau/ConstraintSolver.h)
//!   - type_ref <- method ConstraintGenerator::run (Analysis/src/ConstraintGenerator.cpp)
//!   - type_ref <- method ConstraintGenerator::runOnFragment (Analysis/src/ConstraintGenerator.cpp)
//!   - type_ref <- method ConstraintSolver::ConstraintSolver (Analysis/src/ConstraintSolver.cpp)
//!   - type_ref <- function check (Analysis/src/Frontend.cpp)
//! - outgoing:
//!   - type_ref -> record Scope (Analysis/include/Luau/Scope.h)
//!   - type_ref -> type_alias ConstraintPtr (Analysis/include/Luau/Constraint.h)
//!   - type_ref -> record TypeIds (Analysis/include/Luau/TypeIds.h)
//!   - type_ref -> record DenseHashMap (Common/include/Luau/DenseHash.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item ConstraintSet

extern crate alloc;

use alloc::vec::Vec;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{scope::Scope, type_error::TypeError, type_ids::TypeIds},
  type_aliases::{constraint_ptr::ConstraintPtr, type_id::TypeId},
};

#[derive(Debug)]
pub struct ConstraintSet {
  pub root_scope: *mut Scope,
  pub constraints: Vec<ConstraintPtr>,
  // The set of all free types created during constraint generation
  pub free_types: TypeIds,
  // Map a function's signature scope back to its signature type. Once we've
  // dispatched all of the constraints pertaining to a particular free type,
  // we use this mapping to generalize that free type.
  pub scope_to_function: DenseHashMap<*mut Scope, TypeId>,
  // It is pretty uncommon for constraint generation to itself produce errors, but it can happen.
  pub errors: Vec<TypeError>,
}
