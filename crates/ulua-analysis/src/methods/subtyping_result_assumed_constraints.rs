use alloc::vec::Vec;

use crate::{records::subtyping_result::SubtypingResult, type_aliases::constraint_v::ConstraintV};

impl SubtypingResult {
  pub fn assumed_constraints(&self) -> &Vec<ConstraintV> {
    &self.assumed_constraints
  }
}
