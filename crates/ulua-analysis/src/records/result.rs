use alloc::vec::Vec;
use core::ptr::null_mut;

use crate::{
  enums::unify_result::UnifyResult,
  type_aliases::{constraint_v::ConstraintV, upper_bounds::UpperBounds},
};
#[derive(Debug, Clone)]
pub struct Result {
  pub unified: UnifyResult,
  pub outstanding_constraints: Vec<ConstraintV>,
  pub upper_bound_contributors: UpperBounds,
}

impl Default for Result {
  fn default() -> Self {
    Self {
      unified: UnifyResult::Ok,
      outstanding_constraints: Vec::new(),
      upper_bound_contributors: UpperBounds::new(null_mut()),
    }
  }
}

unsafe impl Send for Result {}
unsafe impl Sync for Result {}
