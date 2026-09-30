use crate::{records::with_predicate::WithPredicate, type_aliases::predicate_vec::PredicateVec};

impl<T: Default> WithPredicate<T> {
  pub fn new() -> Self {
    Self {
      r#type: T::default(),
      predicates: PredicateVec::default(),
    }
  }
}

impl<T: Default> Default for WithPredicate<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> WithPredicate<T> {
  pub fn with_predicate_t(r#type: T) -> Self {
    Self {
      r#type,
      predicates: PredicateVec::default(),
    }
  }

  pub fn with_predicate_t_predicate_vec(r#type: T, predicates: PredicateVec) -> Self {
    Self { r#type, predicates }
  }
}
