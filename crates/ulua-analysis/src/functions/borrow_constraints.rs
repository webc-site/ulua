use alloc::vec::Vec;

use crate::{records::constraint::Constraint, type_aliases::constraint_ptr::ConstraintPtr};
pub fn borrow_constraints(constraints: &[ConstraintPtr]) -> Vec<*mut Constraint> {
  let mut result: Vec<*mut Constraint> = Vec::with_capacity(constraints.len());

  for &c in constraints.iter() {
    result.push(c);
  }

  result
}
