//! Source: `Analysis/include/Luau/Predicate.h:22` (hand-ported)
use crate::{
  macros::variant_member,
  records::{
    and_predicate::AndPredicate, eq_predicate::EqPredicate, is_a_predicate::IsAPredicate,
    not_predicate::NotPredicate, or_predicate::OrPredicate, truthy_predicate::TruthyPredicate,
    type_guard_predicate::TypeGuardPredicate,
  },
};

#[derive(Debug, Clone)]
pub enum Predicate {
  Truthy(TruthyPredicate),
  IsA(IsAPredicate),
  TypeGuard(TypeGuardPredicate),
  Eq(EqPredicate),
  And(AndPredicate),
  Or(OrPredicate),
  Not(NotPredicate),
}

impl Predicate {
  /// C++ `v.index()` — the member's position in the Variant<...> list.
  pub fn index(&self) -> i32 {
    match self {
      Predicate::Truthy(_) => 0,
      Predicate::IsA(_) => 1,
      Predicate::TypeGuard(_) => 2,
      Predicate::Eq(_) => 3,
      Predicate::And(_) => 4,
      Predicate::Or(_) => 5,
      Predicate::Not(_) => 6,
    }
  }
}

variant_member! {
  /// `get_if<T>(&v)` — the Rust shape of C++ overload-on-T over this variant.
  enum PredicateMember: Predicate {
    Truthy => TruthyPredicate,
    IsA => IsAPredicate,
    TypeGuard => TypeGuardPredicate,
    Eq => EqPredicate,
    And => AndPredicate,
    Or => OrPredicate,
    Not => NotPredicate,
  }
}
