use core::mem::swap;

use crate::{
  enums::subtyping_suppression_policy::SubtypingSuppressionPolicy,
  functions::merge_reasonings::merge_reasonings, records::subtyping_result::SubtypingResult,
};
impl SubtypingResult {
  pub fn and_also(
    &mut self,
    mut other: SubtypingResult,
    policy: SubtypingSuppressionPolicy,
  ) -> &mut Self {
    // If the other result is not a subtype, we want to join all of its
    // reasonings to this one. If this result already has reasonings of its own,
    // those need to be attributed here whenever this _also_ failed.
    if !other.is_subtype {
      if self.is_subtype {
        swap(&mut self.reasoning, &mut other.reasoning);
      } else {
        // NOTE: This probably doesn't need to be two copies.
        self.reasoning = merge_reasonings(&self.reasoning, &other.reasoning);
      }
    }

    self.is_subtype &= other.is_subtype;

    if policy == SubtypingSuppressionPolicy::All {
      self.is_error_suppressing &= other.is_error_suppressing;
    } else {
      self.is_error_suppressing |= other.is_error_suppressing;
    }

    self.normalization_too_complex |= other.normalization_too_complex;
    self.is_cacheable &= other.is_cacheable;

    self.errors.extend(other.errors);
    self
      .generic_bounds_mismatches
      .extend(other.generic_bounds_mismatches);
    self.assumed_constraints.extend(other.assumed_constraints);

    self
  }
}
