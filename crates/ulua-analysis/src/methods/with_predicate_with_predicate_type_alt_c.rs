use crate::{records::with_predicate::WithPredicate, type_aliases::predicate_vec::PredicateVec};

impl<T> WithPredicate<T> {
  pub fn with_predicate_t_predicate_vec(r#type: T, predicates: PredicateVec) -> Self {
    Self { r#type, predicates }
  }
}
