//! Source: `Analysis/src/ControlFlowGraph.cpp:38-49` (hand-ported)
//! C++ `RefinementId RefinementArena::negation(RefinementId r)`.
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    arena_handle::Handle, conjunction_control_flow_graph::Conjunction,
    disjunction_control_flow_graph::Disjunction, negation_control_flow_graph::Negation,
    proposition_control_flow_graph::Proposition,
    refinement_arena_control_flow_graph::RefinementArena,
  },
  type_aliases::{
    refinement_control_flow_graph::{Refinement, RefinementMember},
    refinement_id_control_flow_graph::RefinementId,
  },
};

impl RefinementArena {
  /// 前置条件由类型编码：`r: RefinementId = Handle<Refinement>` 恒非空且指向
  /// 本 arena 分配象（C++ `NotNull<Refinement>` 同义）。`Handle::get` 物化的
  /// 共享只读借用跨越下方递归仍成立，因为该 arena 只追加、bump 块地址永不
  /// 移动，既有节点在消解期间不被改写或释放；Conjunction/Disjunction 分支
  /// 只对严格更小的子树递归（构造期 DeMorgan 展开保证无环）。
  pub fn negation_mut(&mut self, r: RefinementId) -> RefinementId {
    // C++ `get<T>(r)` == `get_if<T>(r.get())`.
    let refinement: &Refinement = r.get();

    // if (auto* conj = get<Conjunction>(r))
    //     return disjunction(negation(conj->lhs), negation(conj->rhs));
    if let Some(conj) = <Conjunction as RefinementMember>::get_if(refinement) {
      // lhs/rhs 为 Copy 句柄，按值拷出后借用即结束。
      let (lhs, rhs) = (conj.lhs, conj.rhs);
      let nl = self.negation_mut(lhs);
      let nr = self.negation_mut(rhs);
      return self.disjunction_mut(nl, nr);
    }

    // if (auto* disj = get<Disjunction>(r))
    //     return conjunction(negation(disj->lhs), negation(disj->rhs));
    if let Some(disj) = <Disjunction as RefinementMember>::get_if(refinement) {
      let (lhs, rhs) = (disj.lhs, disj.rhs);
      let nl = self.negation_mut(lhs);
      let nr = self.negation_mut(rhs);
      return self.conjunction_mut(nl, nr);
    }

    // if (auto* neg = get<Negation>(r))
    //     return neg->refinement;
    if let Some(neg) = <Negation as RefinementMember>::get_if(refinement) {
      return neg.refinement;
    }

    // LUAU_ASSERT(get<Proposition>(r));
    LUAU_ASSERT!(<Proposition as RefinementMember>::get_if(refinement).is_some());
    // return NotNull{allocator.allocate(Negation{r})};
    Handle::from_ptr(
      self
        .allocator
        .allocate(Refinement::Negation(Negation { refinement: r })),
    )
  }
}
