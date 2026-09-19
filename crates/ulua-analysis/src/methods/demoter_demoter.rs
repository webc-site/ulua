use crate::records::{builtin_types::BuiltinTypes, demoter::Demoter, type_arena::TypeArena};

impl Demoter {
  pub fn demoter(&mut self, arena: *mut TypeArena, builtins: *mut BuiltinTypes) {
    self.arena = arena;
    self.builtins = builtins;
  }
}
