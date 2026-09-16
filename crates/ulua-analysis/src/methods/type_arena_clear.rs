use crate::records::type_arena::TypeArena;

impl TypeArena {
  pub fn clear(&mut self) {
    self.types.clear();
    self.type_packs.clear();
  }
}
