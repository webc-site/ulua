//! Source: `Analysis/src/Refinement.cpp:36-42` (hand-ported)
//! C++ `RefinementId RefinementArena::disjunction(RefinementId lhs, RefinementId rhs)`.
//!
//! The faithful body lives on the canonical inherent method
//! `RefinementArena::disjunction` (see `records/refinement_arena_refinement.rs`).
//! This signature-disambiguated entry point mirrors the
//! `proposition_refinement_key_type_id` precedent for this arena and delegates.
use crate::{
  records::refinement_arena_refinement::RefinementArena,
  type_aliases::refinement_id_refinement::RefinementId,
};

impl RefinementArena {
  pub fn disjunction_refinement_id_refinement_id(
    &mut self,
    lhs: RefinementId,
    rhs: RefinementId,
  ) -> RefinementId {
    self.disjunction(lhs, rhs)
  }
}
