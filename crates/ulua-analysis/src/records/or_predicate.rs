//! Source: `Analysis/include/Luau/Predicate.h`

use crate::type_aliases::predicate_vec::PredicateVec;

#[derive(Debug, Clone)]
pub struct OrPredicate {
  pub lhs: PredicateVec,
  pub rhs: PredicateVec,
}
