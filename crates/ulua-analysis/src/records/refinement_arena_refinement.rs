//! Source: `Analysis/src/Refinement.cpp` (hand-ported)
use core::ptr::null_mut;

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
  pub fn variadic(&mut self, refis: &[RefinementId]) -> RefinementId {
    // bool hasRefinements = false;
    // for (RefinementId r : refis)
    //     hasRefinements |= bool(r);
    let mut has_refinements = false;
    for r in refis {
      has_refinements |= !r.is_null();
    }

    // if (!hasRefinements)
    //     return nullptr;
    if !has_refinements {
      return null_mut();
    }

    // return NotNull{allocator.allocate(Variadic{refis})};
    self.allocator.allocate(Refinement::Variadic(Variadic {
      refinements: refis.to_vec(),
    }))
  }

  // Analysis/src/Refinement.cpp:20 — RefinementId RefinementArena::negation(RefinementId refinement)
  pub fn negation(&mut self, refinement: RefinementId) -> RefinementId {
    // if (!refinement)
    //     return nullptr;
    if refinement.is_null() {
      return null_mut();
    }

    // return NotNull{allocator.allocate(Negation{refinement})};
    self
      .allocator
      .allocate(Refinement::Negation(Negation { refinement }))
  }

  // Analysis/src/Refinement.cpp:28 — RefinementId RefinementArena::conjunction(RefinementId lhs, RefinementId rhs)
  pub fn conjunction(&mut self, lhs: RefinementId, rhs: RefinementId) -> RefinementId {
    // if (!lhs && !rhs)
    //     return nullptr;
    if lhs.is_null() && rhs.is_null() {
      return null_mut();
    }

    // return NotNull{allocator.allocate(Conjunction{lhs, rhs})};
    self
      .allocator
      .allocate(Refinement::Conjunction(Conjunction { lhs, rhs }))
  }

  // Analysis/src/Refinement.cpp:36 — RefinementId RefinementArena::disjunction(RefinementId lhs, RefinementId rhs)
  pub fn disjunction(&mut self, lhs: RefinementId, rhs: RefinementId) -> RefinementId {
    // if (!lhs && !rhs)
    //     return nullptr;
    if lhs.is_null() && rhs.is_null() {
      return null_mut();
    }

    // return NotNull{allocator.allocate(Disjunction{lhs, rhs})};
    self
      .allocator
      .allocate(Refinement::Disjunction(Disjunction { lhs, rhs }))
  }

  // Analysis/src/Refinement.cpp:44 — RefinementId RefinementArena::equivalence(RefinementId lhs, RefinementId rhs)
  pub fn equivalence(&mut self, lhs: RefinementId, rhs: RefinementId) -> RefinementId {
    // if (!lhs && !rhs)
    //     return nullptr;
    if lhs.is_null() && rhs.is_null() {
      return null_mut();
    }

    // return NotNull{allocator.allocate(Equivalence{lhs, rhs})};
    self
      .allocator
      .allocate(Refinement::Equivalence(Equivalence { lhs, rhs }))
  }

  // Analysis/src/Refinement.cpp:52 — RefinementId RefinementArena::proposition(const RefinementKey* key, TypeId discriminant_ty)
  pub fn proposition(
    &mut self,
    key: *const RefinementKey,
    discriminant_ty: TypeId,
  ) -> RefinementId {
    // if (!key)
    //     return nullptr;
    if key.is_null() {
      return null_mut();
    }

    // return NotNull{allocator.allocate(Proposition{key, discriminant_ty, false})};
    self
      .allocator
      .allocate(Refinement::Proposition(Proposition {
        key,
        discriminant_ty,
        implicit_from_call: false,
      }))
  }

  // Analysis/src/Refinement.cpp:60 — RefinementId RefinementArena::implicitProposition(const RefinementKey* key, TypeId discriminant_ty)
}
