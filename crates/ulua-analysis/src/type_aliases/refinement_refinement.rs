//! Generated skeleton item.
//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/Refinement.h:21:refinement`
//! Source: `Analysis/include/Luau/Refinement.h`
//! Graph edges:
//! - declared_by: source_file Analysis/include/Luau/Refinement.h
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypedAllocator.h
//!   - includes -> source_file Common/include/Luau/Variant.h
//!   - includes -> source_file Analysis/include/Luau/TypeFwd.h
//! - incoming:
//!   - declares <- source_file Analysis/include/Luau/Refinement.h
//!   - type_ref <- type_alias Refinement (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref <- type_alias RefinementId (Analysis/include/Luau/Refinement.h)
//!   - type_ref <- record RefinementArena (Analysis/include/Luau/Refinement.h)
//! - outgoing:
//!   - type_ref -> type_alias Refinement (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record Variadic (Analysis/include/Luau/Refinement.h)
//!   - type_ref -> record Negation (Analysis/include/Luau/Refinement.h)
//!   - type_ref -> record Conjunction (Analysis/include/Luau/Refinement.h)
//!   - type_ref -> record Disjunction (Analysis/include/Luau/Refinement.h)
//!   - type_ref -> record Equivalence (Analysis/include/Luau/Refinement.h)
//!   - type_ref -> record Proposition (Analysis/include/Luau/Refinement.h)
//!   - translates_to -> rust_item Refinement

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
