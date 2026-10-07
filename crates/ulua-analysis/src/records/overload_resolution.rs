use alloc::vec::Vec;

use ulua_common::records::{dense_hash_set::DenseHashSet, variant::Variant2};

use crate::type_aliases::{
  constraint_v::ConstraintV, error_vec::ErrorVec, subtyping_reasonings::SubtypingReasonings,
  type_id::TypeId,
};

/// C++ `IncompatibilityReason`（Analysis/include/Luau/OverloadResolution.h）
pub type IncompatibilityReason = Variant2<SubtypingReasonings, ErrorVec>;

#[derive(Debug, Clone)]
pub struct OverloadResolution {
  pub ok: Vec<TypeId>,
  pub non_functions: Vec<TypeId>,
  pub potential_overloads: Vec<(TypeId, Vec<ConstraintV>)>,
  pub incompatible_overloads: Vec<(TypeId, IncompatibilityReason)>,
  pub arity_mismatches: Vec<TypeId>,
  pub metamethods: DenseHashSet<TypeId>,
}
