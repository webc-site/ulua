//! @interface-stub
use core::ptr::{null, null_mut};

use ulua_common::records::dense_hash_table::DenseDefault;

use crate::type_aliases::{refinement_id_refinement::RefinementId, type_id::TypeId};
#[derive(Debug, Clone)]
pub struct Inference {
  pub ty: TypeId,
  pub refinement: RefinementId,
}

impl DenseDefault for Inference {
  fn dense_default() -> Self {
    Self {
      ty: null(),
      refinement: null_mut(),
    }
  }
}

impl Inference {
  // C++: `Inference()` — `ty` default-initialized to nullptr, no refinement.
  // (Analysis/include/Luau/ConstraintGenerator.h)
  pub fn new() -> Self {
    Self {
      ty: null(),
      refinement: null_mut(),
    }
  }
  // C++: `Inference(TypeId ty, RefinementId refinement = nullptr)`.
  pub fn inference_type_id_refinement_id(ty: TypeId, refinement: RefinementId) -> Self {
    Self { ty, refinement }
  }
}

impl Default for Inference {
  fn default() -> Self {
    Self::new()
  }
}
