use core::mem::take;

use crate::{
  functions::merge_reasonings::merge_reasonings, records::subtyping_result::SubtypingResult,
};
impl SubtypingResult {
  pub fn or_else(&mut self, mut other: SubtypingResult) -> &mut Self {
    // If this result is a subtype, we do not join the reasoning lists. If this
    // result is not a subtype, but the other is a subtype, we want to _clear_
    // our reasoning list. If both results are not subtypes, we join the
    // reasoning lists.
    if !self.is_subtype {
      if other.is_subtype {
        self.reasoning.clear();
        self.assumed_constraints = take(&mut other.assumed_constraints);
      } else {
        self.reasoning = merge_reasonings(&self.reasoning, &other.reasoning);
        self.is_error_suppressing |= other.is_error_suppressing;
      }
    } else if other.is_subtype {
      // If the other result has assumed constraints, we drop ours (given
      // we represent a failed subtype) and then take the constraints of
      // the other check.
      self.assumed_constraints = take(&mut other.assumed_constraints);
    }

    self.is_subtype |= other.is_subtype;
    self.normalization_too_complex |= other.normalization_too_complex;
    self.is_cacheable &= other.is_cacheable;
    self.errors.extend(other.errors);
    self
      .generic_bounds_mismatches
      .extend(other.generic_bounds_mismatches);

    self
  }
}
