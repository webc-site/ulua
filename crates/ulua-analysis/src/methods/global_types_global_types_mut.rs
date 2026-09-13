use crate::records::{global_types::GlobalTypes, type_arena::TypeArena};

impl GlobalTypes {
  pub fn global_types_mut(&mut self) -> &mut TypeArena {
    &mut self.global_types
  }
}
