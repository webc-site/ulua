//! Source: `Analysis/src/Refinement.cpp:20-26` (hand-ported)
//! C++ `RefinementId RefinementArena::negation(RefinementId refinement)`.
//!
//! The faithful body lives on the canonical inherent method
//! `RefinementArena::negation` (see `records/refinement_arena_refinement.rs`).
//! This signature-disambiguated entry point mirrors the
//! `proposition_refinement_key_type_id` precedent for this arena and delegates.
use crate::{
  records::refinement_arena_refinement::RefinementArena,
  type_aliases::refinement_id_refinement::RefinementId,
};

impl RefinementArena {
  pub fn negation_refinement_id(&mut self, refinement: RefinementId) -> RefinementId {
    self.negation(refinement)
  }
}
