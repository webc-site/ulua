//! Source: `Analysis/src/ControlFlowGraph.cpp:18-54` (hand-ported)

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    arena_handle::Handle, conjunction_control_flow_graph::Conjunction,
    disjunction_control_flow_graph::Disjunction, negation_control_flow_graph::Negation,
    proposition_control_flow_graph::Proposition, typed_allocator::TypedAllocator,
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
    // Safety 说明：`allocate` 恒返回 arena 块界内的非空槽位（append_block 对
    // null 块已按 bad_alloc 语义 panic），`from_ptr` 的非空断言必成立。
    Handle::from_ptr(
      self
        .allocator
        .allocate(Refinement::Proposition(Proposition {
          ptr: def,
          r#type: None,
          is_typeof: false,
          sense,
        })),
    )
  }

  // RefinementId RefinementArena::typeProposition(DefId def, std::optional<std::string> type, bool is_typeof, bool sense)

  // RefinementId RefinementArena::conjunction(RefinementId lhs, RefinementId rhs)
  pub fn conjunction(&mut self, lhs: RefinementId, rhs: RefinementId) -> RefinementId {
    // return NotNull{allocator.allocate(Conjunction{lhs, rhs})};
    Handle::from_ptr(
      self
        .allocator
        .allocate(Refinement::Conjunction(Conjunction { lhs, rhs })),
    )
  }

  // RefinementId RefinementArena::disjunction(RefinementId lhs, RefinementId rhs)
  pub fn disjunction(&mut self, lhs: RefinementId, rhs: RefinementId) -> RefinementId {
    // return NotNull{allocator.allocate(Disjunction{lhs, rhs})};
    Handle::from_ptr(
      self
        .allocator
        .allocate(Refinement::Disjunction(Disjunction { lhs, rhs })),
    )
  }

  /// 前置条件由类型编码：`r: RefinementId = Handle<Refinement>` 恒非空且指向
  /// 本 arena 分配象（对应 C++ `NotNull<Refinement>`），以 `r` 为根的
  /// refinement 表达式树无环——Conjunction/Disjunction 分支会对 `lhs`/`rhs`
  /// 递归下探，环状结构将无限递归（构造期 DeMorgan 展开只向更小子树递归）。
  /// 另需单线程访问（本方法经 `&mut self` 向 arena 追加 Negation 节点）。
  // RefinementId RefinementArena::negation(RefinementId r)
  pub fn negation(&mut self, r: RefinementId) -> RefinementId {
    // `get<T>(r)` is `get_if<T>(r.get())` over r's variant.
    // if (auto* conj = get<Conjunction>(r))
    //     return disjunction(negation(conj->lhs), negation(conj->rhs));
    // `Handle::get` 物化共享只读借用（与原 `&*r` 同构）；读到的 lhs/rhs
    // 为 Copy 句柄，按值拷出后借用即结束。
    if let Some((lhs, rhs)) = Conjunction::get_if(r.get()).map(|c| (c.lhs, c.rhs)) {
      let nl = self.negation(lhs);
      let nr = self.negation(rhs);
      return self.disjunction(nl, nr);
    }
    // if (auto* disj = get<Disjunction>(r))
    //     return conjunction(negation(disj->lhs), negation(disj->rhs));
    if let Some((lhs, rhs)) = Disjunction::get_if(r.get()).map(|d| (d.lhs, d.rhs)) {
      let nl = self.negation(lhs);
      let nr = self.negation(rhs);
      return self.conjunction(nl, nr);
    }
    // if (auto* neg = get<Negation>(r))
    //     return neg->refinement;
    if let Some(refinement) = Negation::get_if(r.get()).map(|n| n.refinement) {
      return refinement;
    }

    // LUAU_ASSERT(get<Proposition>(r));
    LUAU_ASSERT!(Proposition::get_if(r.get()).is_some());
    // return NotNull{allocator.allocate(Negation{r})};
    Handle::from_ptr(
      self
        .allocator
        .allocate(Refinement::Negation(Negation { refinement: r })),
    )
  }

  // void RefinementArena::freeze()
  pub fn freeze(&mut self) {
    // allocator.freeze();
    self.allocator.freeze();
  }
}
