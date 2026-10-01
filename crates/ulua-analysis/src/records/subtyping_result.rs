use alloc::vec::Vec;

use crate::{
  records::generic_bounds_mismatch::GenericBoundsMismatch,
  type_aliases::{
    constraint_v::ConstraintV, error_vec::ErrorVec, subtyping_reasonings::SubtypingReasonings,
  },
};
#[derive(Debug, Clone)]
pub struct SubtypingResult {
  pub(crate) is_subtype: bool,
  pub(crate) normalization_too_complex: bool,
  pub(crate) is_cacheable: bool,
  pub(crate) is_error_suppressing: bool,
  pub(crate) errors: ErrorVec,
  pub(crate) reasoning: SubtypingReasonings,
  pub(crate) assumed_constraints: Vec<ConstraintV>,
  pub(crate) generic_bounds_mismatches: Vec<GenericBoundsMismatch>,
}

impl SubtypingResult {
  #[inline]
  pub fn ok() -> Self {
    Self {
      is_subtype: true,
      ..Default::default()
    }
  }

  #[inline]
  pub fn fail() -> Self {
    Self::default()
  }

  #[inline]
  pub fn too_complex() -> Self {
    Self {
      is_subtype: false,
      normalization_too_complex: true,
      ..Default::default()
    }
  }

  #[inline]
  pub fn uncacheable_ok() -> Self {
    Self {
      is_subtype: true,
      is_cacheable: false,
      ..Default::default()
    }
  }

  #[inline]
  pub fn uncacheable_fail() -> Self {
    Self {
      is_subtype: false,
      is_cacheable: false,
      ..Default::default()
    }
  }

  #[inline]
  pub fn uncacheable(is_subtype: bool) -> Self {
    Self {
      is_subtype,
      is_cacheable: false,
      ..Default::default()
    }
  }

  #[inline]
  pub fn from_is_subtype(is_subtype: bool) -> Self {
    Self {
      is_subtype,
      ..Default::default()
    }
  }
}
