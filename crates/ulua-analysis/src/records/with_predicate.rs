use crate::type_aliases::predicate_vec::PredicateVec;

#[derive(Debug, Clone)]
pub struct WithPredicate<T> {
  pub(crate) r#type: T,
  pub(crate) predicates: PredicateVec,
}
