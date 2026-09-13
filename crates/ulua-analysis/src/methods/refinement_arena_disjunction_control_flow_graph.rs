//! Source: `Analysis/src/ControlFlowGraph.cpp:33-36` (hand-ported)
//! C++ `RefinementId RefinementArena::disjunction(RefinementId lhs, RefinementId rhs)`.
use crate::{
  records::{
    disjunction_control_flow_graph::Disjunction,
    refinement_arena_control_flow_graph::RefinementArena,
  },
  type_aliases::{
    refinement_control_flow_graph::Refinement, refinement_id_control_flow_graph::RefinementId,
  },
};

impl RefinementArena {
  pub fn disjunction_mut(&mut self, lhs: RefinementId, rhs: RefinementId) -> RefinementId {
    // C++: return NotNull{allocator.allocate(Disjunction{lhs, rhs})};
    self
      .allocator
      .allocate(Refinement::Disjunction(Disjunction { lhs, rhs }))
  }
}
