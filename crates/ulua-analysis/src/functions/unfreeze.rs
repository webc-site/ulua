use ulua_common::FFlag;

use crate::records::type_arena::TypeArena;

pub fn unfreeze(arena: &mut TypeArena) {
  if !FFlag::DebugLuauFreezeArena.get() {
    return;
  }

  arena.types.unfreeze();
  arena.type_packs.unfreeze();
}
