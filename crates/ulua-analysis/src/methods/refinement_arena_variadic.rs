//! Source: `Analysis/src/Refinement.cpp:8-18` (hand-ported)
//! C++ `RefinementId RefinementArena::variadic(const std::vector<RefinementId>& refis)`.
//!
//! The faithful body lives on the canonical inherent method
//! `RefinementArena::variadic` (see `records/refinement_arena_refinement.rs`).
//! This signature-disambiguated entry point mirrors the
//! `proposition_refinement_key_type_id` precedent for this arena and delegates.
use crate::{
  records::refinement_arena_refinement::RefinementArena,
  type_aliases::refinement_id_refinement::RefinementId,
};

impl RefinementArena {
  pub fn variadic_refinement_ids(&mut self, refis: &[RefinementId]) -> RefinementId {
    self.variadic(refis)
  }
}
