//! Source: `Analysis/src/Refinement.cpp` (hand-ported)
//!
//! B 型（arena 结点可空句柄契约）：`RefinementId` 即 `*mut Refinement`，在
//! type_aliases 处声明为可空（cpp Refinement.h:22 `(can be null)`）。本 arena 各
//! 组合子的「空指针 = 无可组合 refinement」是退化输入的缺省返回：§2 收口后返回面
//! 统一为 `Option<RefinementId>`，cpp `return nullptr` 对应 `None`，本文件不再制造
//! `null_mut()` 哨兵（非 C-ABI 边界）。入参面仍是可空句柄（结点字段直存可空 id，
//! 落点为 TypedAllocator bump 结点布局，唯一写入方为本 arena）；消费方
//! （constraint_generator 的 apply_refinements/negation 传递链）对 `None`/空句柄
//! 按 no-op 处理，不解引用。
use crate::{
  records::{
    conjunction_refinement::Conjunction, disjunction_refinement::Disjunction,
    equivalence::Equivalence, negation_refinement::Negation, proposition_refinement::Proposition,
    refinement_key::RefinementKey, typed_allocator::TypedAllocator, variadic::Variadic,
  },
  type_aliases::{
    refinement_id_refinement::RefinementId, refinement_refinement::Refinement, type_id::TypeId,
  },
};
#[derive(Debug)]
pub struct RefinementArena {
  pub(crate) allocator: TypedAllocator<Refinement>,
}

impl RefinementArena {
  // Analysis/src/Refinement.cpp:8 — RefinementId RefinementArena::variadic(const std::vector<RefinementId>& refis)
  pub fn variadic(&mut self, refis: &[RefinementId]) -> Option<RefinementId> {
    // bool hasRefinements = false;
    // for (RefinementId r : refis)
    //     hasRefinements |= bool(r);
    let has_refinements = refis.iter().any(|r| !r.is_null());

    // if (!hasRefinements)
    //     return nullptr;
    if !has_refinements {
      return None;
    }

    // return NotNull{allocator.allocate(Variadic{refis})};
    Some(self.allocator.allocate(Refinement::Variadic(Variadic {
      refinements: refis.to_vec(),
    })))
  }

  // Analysis/src/Refinement.cpp:20 — RefinementId RefinementArena::negation(RefinementId refinement)
  pub fn negation(&mut self, refinement: RefinementId) -> Option<RefinementId> {
    // if (!refinement)
    //     return nullptr;
    let refinement = Some(refinement).filter(|r| !r.is_null())?;

    // return NotNull{allocator.allocate(Negation{refinement})};
    Some(
      self
        .allocator
        .allocate(Refinement::Negation(Negation { refinement })),
    )
  }

  // Analysis/src/Refinement.cpp:28 — RefinementId RefinementArena::conjunction(RefinementId lhs, RefinementId rhs)
  pub fn conjunction(&mut self, lhs: RefinementId, rhs: RefinementId) -> Option<RefinementId> {
    // if (!lhs && !rhs)
    //     return nullptr;
    if lhs.is_null() && rhs.is_null() {
      return None;
    }

    // return NotNull{allocator.allocate(Conjunction{lhs, rhs})};
    Some(
      self
        .allocator
        .allocate(Refinement::Conjunction(Conjunction { lhs, rhs })),
    )
  }

  // Analysis/src/Refinement.cpp:36 — RefinementId RefinementArena::disjunction(RefinementId lhs, RefinementId rhs)
  pub fn disjunction(&mut self, lhs: RefinementId, rhs: RefinementId) -> Option<RefinementId> {
    // if (!lhs && !rhs)
    //     return nullptr;
    if lhs.is_null() && rhs.is_null() {
      return None;
    }

    // return NotNull{allocator.allocate(Disjunction{lhs, rhs})};
    Some(
      self
        .allocator
        .allocate(Refinement::Disjunction(Disjunction { lhs, rhs })),
    )
  }

  // Analysis/src/Refinement.cpp:44 — RefinementId RefinementArena::equivalence(RefinementId lhs, RefinementId rhs)
  pub fn equivalence(&mut self, lhs: RefinementId, rhs: RefinementId) -> Option<RefinementId> {
    // if (!lhs && !rhs)
    //     return nullptr;
    if lhs.is_null() && rhs.is_null() {
      return None;
    }

    // return NotNull{allocator.allocate(Equivalence{lhs, rhs})};
    Some(
      self
        .allocator
        .allocate(Refinement::Equivalence(Equivalence { lhs, rhs })),
    )
  }

  // Analysis/src/Refinement.cpp:52 — RefinementId RefinementArena::proposition(const RefinementKey* key, TypeId discriminant_ty)
  pub fn proposition(
    &mut self,
    key: *const RefinementKey,
    discriminant_ty: TypeId,
  ) -> Option<RefinementId> {
    // if (!key)
    //     return nullptr;
    if key.is_null() {
      return None;
    }

    // return NotNull{allocator.allocate(Proposition{key, discriminant_ty, false})};
    Some(
      self
        .allocator
        .allocate(Refinement::Proposition(Proposition {
          key,
          discriminant_ty,
          implicit_from_call: false,
        })),
    )
  }

  // Analysis/src/Refinement.cpp:60 — RefinementId RefinementArena::implicitProposition(const RefinementKey* key, TypeId discriminant_ty)
}
