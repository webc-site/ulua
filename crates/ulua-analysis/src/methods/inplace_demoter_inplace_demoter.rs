use crate::records::{
  inplace_demoter::InplaceDemoter, type_arena::TypeArena, type_level::TypeLevel,
};

impl InplaceDemoter {
  pub fn inplace_demoter(&mut self, level: TypeLevel, arena: *mut TypeArena) {
    self.new_level = level;
    self.arena = arena;
  }
}
