//! Source: `Analysis/include/Luau/Predicate.h`

use crate::type_aliases::predicate_vec::PredicateVec;

#[derive(Debug, Clone)]
pub struct AndPredicate {
  pub lhs: PredicateVec,
  pub rhs: PredicateVec,
}
