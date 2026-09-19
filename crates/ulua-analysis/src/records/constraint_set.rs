//! Source: `Analysis/include/Luau/ConstraintSet.h`

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
