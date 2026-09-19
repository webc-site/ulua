//! Source: `Analysis/include/Luau/Refinement.h`

// Refinement.h:21 — using Refinement = Variant<Variadic, Negation, Conjunction,
//                                              Disjunction, Equivalence, Proposition>
use crate::records::{
  conjunction_refinement::Conjunction, disjunction_refinement::Disjunction,
  equivalence::Equivalence, negation_refinement::Negation, proposition_refinement::Proposition,
  variadic::Variadic,
};

#[derive(Debug, Clone)]
pub enum Refinement {
  Variadic(Variadic),
  Negation(Negation),
  Conjunction(Conjunction),
  Disjunction(Disjunction),
  Equivalence(Equivalence),
  Proposition(Proposition),
}

/// `get_if<T>(refinement.get())` over the refinement variant.
pub trait RefinementMember: Sized {
  fn get_if(v: &Refinement) -> Option<&Self>;
  fn get_if_mut(v: &mut Refinement) -> Option<&mut Self>;
}

impl RefinementMember for Variadic {
  fn get_if(v: &Refinement) -> Option<&Self> {
    match v {
      Refinement::Variadic(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut Refinement) -> Option<&mut Self> {
    match v {
      Refinement::Variadic(x) => Some(x),
      _ => None,
    }
  }
}

impl RefinementMember for Negation {
  fn get_if(v: &Refinement) -> Option<&Self> {
    match v {
      Refinement::Negation(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut Refinement) -> Option<&mut Self> {
    match v {
      Refinement::Negation(x) => Some(x),
      _ => None,
    }
  }
}

impl RefinementMember for Conjunction {
  fn get_if(v: &Refinement) -> Option<&Self> {
    match v {
      Refinement::Conjunction(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut Refinement) -> Option<&mut Self> {
    match v {
      Refinement::Conjunction(x) => Some(x),
      _ => None,
    }
  }
}

impl RefinementMember for Disjunction {
  fn get_if(v: &Refinement) -> Option<&Self> {
    match v {
      Refinement::Disjunction(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut Refinement) -> Option<&mut Self> {
    match v {
      Refinement::Disjunction(x) => Some(x),
      _ => None,
    }
  }
}

impl RefinementMember for Equivalence {
  fn get_if(v: &Refinement) -> Option<&Self> {
    match v {
      Refinement::Equivalence(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut Refinement) -> Option<&mut Self> {
    match v {
      Refinement::Equivalence(x) => Some(x),
      _ => None,
    }
  }
}

impl RefinementMember for Proposition {
  fn get_if(v: &Refinement) -> Option<&Self> {
    match v {
      Refinement::Proposition(x) => Some(x),
      _ => None,
    }
  }
  fn get_if_mut(v: &mut Refinement) -> Option<&mut Self> {
    match v {
      Refinement::Proposition(x) => Some(x),
      _ => None,
    }
  }
}
