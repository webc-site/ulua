//! Source: `Analysis/include/Luau/ControlFlowGraph.h:48` (hand-ported)
use crate::{
  macros::variant_enum,
  records::{
    conjunction_control_flow_graph::Conjunction, disjunction_control_flow_graph::Disjunction,
    negation_control_flow_graph::Negation, proposition_control_flow_graph::Proposition,
  },
};

// The NEW dataflow system's Refinement — DISTINCT from Refinement.h's.
variant_enum! {
  #[derive(Debug, Clone)]
  pub enum Refinement {
    Conjunction => Conjunction,
    Disjunction => Disjunction,
    Negation => Negation,
    Proposition => Proposition,
  }

  /// `get_if<T>(&v)` — the Rust shape of C++ overload-on-T over this variant.
  pub trait RefinementMember;
}
