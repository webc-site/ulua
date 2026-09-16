//! Source: `Analysis/src/Refinement.cpp:28-34` (hand-ported)
//! C++ `RefinementId RefinementArena::conjunction(RefinementId lhs, RefinementId rhs)`.
//!
//! The faithful body lives on the canonical inherent method
//! `RefinementArena::conjunction` (see `records/refinement_arena_refinement.rs`).
//! This signature-disambiguated entry point mirrors the
//! `proposition_refinement_key_type_id` precedent for this arena and delegates.
use crate::{
  records::refinement_arena_refinement::RefinementArena,
  type_aliases::refinement_id_refinement::RefinementId,
};

impl RefinementArena {
  pub fn conjunction_refinement_id_refinement_id(
    &mut self,
    lhs: RefinementId,
    rhs: RefinementId,
  ) -> RefinementId {
    self.conjunction(lhs, rhs)
  }
}
