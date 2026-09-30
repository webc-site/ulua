//! Source: `Analysis/include/Luau/ConstraintGenerator.h:49-61` (hand-ported)
use alloc::vec::Vec;
use core::ptr::null;

use crate::type_aliases::{refinement_id_refinement::RefinementId, type_pack_id::TypePackId};
#[derive(Debug, Clone)]
pub struct InferencePack {
  pub tp: TypePackId,
  pub refinements: Vec<RefinementId>,
}

impl InferencePack {
  // C++ `InferencePack() = default;` with `TypePackId tp = nullptr;`.
  pub fn new() -> Self {
    InferencePack {
      tp: null(),
      refinements: Vec::new(),
    }
  }

  // C++ `explicit InferencePack(TypePackId tp, const std::vector<RefinementId>& refinements = {})`.
}

impl Default for InferencePack {
  fn default() -> Self {
    Self::new()
  }
}
