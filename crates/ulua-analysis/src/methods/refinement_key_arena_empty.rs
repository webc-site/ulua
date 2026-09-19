use crate::records::refinement_key_arena::RefinementKeyArena;

impl RefinementKeyArena {
  pub fn empty(&self) -> bool {
    self.allocator.empty()
  }
}
