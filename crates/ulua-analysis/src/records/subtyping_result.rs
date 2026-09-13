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
