use alloc::vec::Vec;

use crate::records::constraint::{Constraint, ConstraintPtr};

pub fn borrow_constraints(constraints: &[ConstraintPtr]) -> Vec<*mut Constraint> {
  constraints.to_vec()
}
