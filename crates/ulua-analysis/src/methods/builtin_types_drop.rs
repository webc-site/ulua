use alloc::boxed::Box;
use core::mem::replace;

use ulua_common::FFlag;

use crate::{
  functions::unfreeze::unfreeze,
  records::{builtin_types::BuiltinTypes, type_arena::TypeArena},
};
impl Drop for BuiltinTypes {
  fn drop(&mut self) {
    let previous = FFlag::DebugLuauFreezeArena.get_global();
    FFlag::DebugLuauFreezeArena.set(self.debug_freeze_arena);

    unfreeze(&mut self.arena);
    let arena = replace(&mut self.arena, Box::new(TypeArena::default()));
    drop(arena);

    FFlag::DebugLuauFreezeArena.set(previous);
  }
}
