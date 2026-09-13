//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/Predicate.h:22:predicate`
//! Source: `Analysis/include/Luau/Predicate.h:22` (hand-ported)
use crate::records::{
  and_predicate::AndPredicate, eq_predicate::EqPredicate, is_a_predicate::IsAPredicate,
  not_predicate::NotPredicate, or_predicate::OrPredicate, truthy_predicate::TruthyPredicate,
  type_guard_predicate::TypeGuardPredicate,
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

/// `get_if<T>(&v)` — the Rust shape of C++ overload-on-T over this variant.
pub trait PredicateMember: Sized {
  fn get_if(v: &Predicate) -> Option<&Self>;
  fn get_if_mut(v: &mut Predicate) -> Option<&mut Self>;
}

impl PredicateMember for TruthyPredicate {
  fn get_if(v: &Predicate) -> Option<&Self> {
    match v {
      Predicate::Truthy(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut Predicate) -> Option<&mut Self> {
    match v {
      Predicate::Truthy(x) => Some(x),
      _ => None,
    }
  }
}

impl PredicateMember for IsAPredicate {
  fn get_if(v: &Predicate) -> Option<&Self> {
    match v {
      Predicate::IsA(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut Predicate) -> Option<&mut Self> {
    match v {
      Predicate::IsA(x) => Some(x),
      _ => None,
    }
  }
}

impl PredicateMember for TypeGuardPredicate {
  fn get_if(v: &Predicate) -> Option<&Self> {
    match v {
      Predicate::TypeGuard(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut Predicate) -> Option<&mut Self> {
    match v {
      Predicate::TypeGuard(x) => Some(x),
      _ => None,
    }
  }
}

impl PredicateMember for EqPredicate {
  fn get_if(v: &Predicate) -> Option<&Self> {
    match v {
      Predicate::Eq(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut Predicate) -> Option<&mut Self> {
    match v {
      Predicate::Eq(x) => Some(x),
      _ => None,
    }
  }
}

impl PredicateMember for AndPredicate {
  fn get_if(v: &Predicate) -> Option<&Self> {
    match v {
      Predicate::And(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut Predicate) -> Option<&mut Self> {
    match v {
      Predicate::And(x) => Some(x),
      _ => None,
    }
  }
}

impl PredicateMember for OrPredicate {
  fn get_if(v: &Predicate) -> Option<&Self> {
    match v {
      Predicate::Or(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut Predicate) -> Option<&mut Self> {
    match v {
      Predicate::Or(x) => Some(x),
      _ => None,
    }
  }
}

impl PredicateMember for NotPredicate {
  fn get_if(v: &Predicate) -> Option<&Self> {
    match v {
      Predicate::Not(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut Predicate) -> Option<&mut Self> {
    match v {
      Predicate::Not(x) => Some(x),
      _ => None,
    }
  }
}
