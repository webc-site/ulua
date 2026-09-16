//! Source: `Analysis/src/ControlFlowGraph.cpp:18-54` (hand-ported)

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    conjunction_control_flow_graph::Conjunction, disjunction_control_flow_graph::Disjunction,
    negation_control_flow_graph::Negation, proposition_control_flow_graph::Proposition,
    typed_allocator::TypedAllocator,
  },
  type_aliases::{
    def_id_control_flow_graph::DefId,
    refinement_control_flow_graph::{Refinement, RefinementMember},
    refinement_id_control_flow_graph::RefinementId,
  },
};

#[derive(Debug)]
pub struct RefinementArena {
  pub(crate) allocator: TypedAllocator<Refinement>,
}

impl RefinementArena {
  // RefinementId RefinementArena::proposition(DefId def, bool sense)
  pub fn proposition(&mut self, def: DefId, sense: bool) -> RefinementId {
    // return NotNull{allocator.allocate(Proposition{def, std::nullopt, /*is_typeof*/ false, sense})};
    self
      .allocator
      .allocate(Refinement::Proposition(Proposition {
        ptr: def,
        r#type: None,
        is_typeof: false,
        sense,
      }))
  }

  // RefinementId RefinementArena::typeProposition(DefId def, std::optional<std::string> type, bool is_typeof, bool sense)

  // RefinementId RefinementArena::conjunction(RefinementId lhs, RefinementId rhs)
  pub fn conjunction(&mut self, lhs: RefinementId, rhs: RefinementId) -> RefinementId {
    // return NotNull{allocator.allocate(Conjunction{lhs, rhs})};
    self
      .allocator
      .allocate(Refinement::Conjunction(Conjunction { lhs, rhs }))
  }

  // RefinementId RefinementArena::disjunction(RefinementId lhs, RefinementId rhs)
  pub fn disjunction(&mut self, lhs: RefinementId, rhs: RefinementId) -> RefinementId {
    // return NotNull{allocator.allocate(Disjunction{lhs, rhs})};
    self
      .allocator
      .allocate(Refinement::Disjunction(Disjunction { lhs, rhs }))
  }

  /// # Safety
  /// 调用方须保证满足 C++ 原实现定义的内部不变量。
  // RefinementId RefinementArena::negation(RefinementId r)
  pub unsafe fn negation(&mut self, r: RefinementId) -> RefinementId {
    // `get<T>(r)` is `get_if<T>(r.get())` over r's variant.
    // if (auto* conj = get<Conjunction>(r))
    //     return disjunction(negation(conj->lhs), negation(conj->rhs));
    if let Some((lhs, rhs)) = unsafe { Conjunction::get_if(&*r) }.map(|c| (c.lhs, c.rhs)) {
      let nl = unsafe { self.negation(lhs) };
      let nr = unsafe { self.negation(rhs) };
      return self.disjunction(nl, nr);
    }
    // if (auto* disj = get<Disjunction>(r))
    //     return conjunction(negation(disj->lhs), negation(disj->rhs));
    if let Some((lhs, rhs)) = unsafe { Disjunction::get_if(&*r) }.map(|d| (d.lhs, d.rhs)) {
      let nl = unsafe { self.negation(lhs) };
      let nr = unsafe { self.negation(rhs) };
      return self.conjunction(nl, nr);
    }
    // if (auto* neg = get<Negation>(r))
    //     return neg->refinement;
    if let Some(refinement) = unsafe { Negation::get_if(&*r) }.map(|n| n.refinement) {
      return refinement;
    }

    // LUAU_ASSERT(get<Proposition>(r));
    LUAU_ASSERT!(unsafe { Proposition::get_if(&*r) }.is_some());
    // return NotNull{allocator.allocate(Negation{r})};
    self
      .allocator
      .allocate(Refinement::Negation(Negation { refinement: r }))
  }

  // void RefinementArena::freeze()
  pub fn freeze(&mut self) {
    // allocator.freeze();
    self.allocator.freeze();
  }
}
