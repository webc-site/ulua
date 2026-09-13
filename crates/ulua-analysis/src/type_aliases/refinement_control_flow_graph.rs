//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/ControlFlowGraph.h:48:refinement`
//! Source: `Analysis/include/Luau/ControlFlowGraph.h:48` (hand-ported)
use crate::records::{
  conjunction_control_flow_graph::Conjunction, disjunction_control_flow_graph::Disjunction,
  negation_control_flow_graph::Negation, proposition_control_flow_graph::Proposition,
};

// The NEW dataflow system's Refinement — DISTINCT from Refinement.h's.
#[derive(Debug, Clone)]
pub enum Refinement {
  Conjunction(Conjunction),
  Disjunction(Disjunction),
  Negation(Negation),
  Proposition(Proposition),
}

impl Refinement {
  /// C++ `v.index()` — the member's position in the Variant<...> list.
  pub fn index(&self) -> i32 {
    match self {
      Refinement::Conjunction(_) => 0,
      Refinement::Disjunction(_) => 1,
      Refinement::Negation(_) => 2,
      Refinement::Proposition(_) => 3,
    }
  }
}

/// `get_if<T>(&v)` — the Rust shape of C++ overload-on-T over this variant.
pub trait RefinementMember: Sized {
  fn get_if(v: &Refinement) -> Option<&Self>;
  fn get_if_mut(v: &mut Refinement) -> Option<&mut Self>;
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
