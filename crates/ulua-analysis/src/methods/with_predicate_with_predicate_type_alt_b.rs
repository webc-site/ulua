use crate::{records::with_predicate::WithPredicate, type_aliases::predicate_vec::PredicateVec};

impl<T> WithPredicate<T> {
  pub fn with_predicate_t(r#type: T) -> Self {
    Self {
      r#type,
      predicates: PredicateVec::default(),
    }
  }
}
