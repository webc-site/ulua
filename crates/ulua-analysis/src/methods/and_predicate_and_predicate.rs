//! @interface-stub
use crate::{records::and_predicate::AndPredicate, type_aliases::predicate_vec::PredicateVec};

impl AndPredicate {
  pub fn new(lhs: PredicateVec, rhs: PredicateVec) -> Self {
    AndPredicate { lhs, rhs }
  }
}
