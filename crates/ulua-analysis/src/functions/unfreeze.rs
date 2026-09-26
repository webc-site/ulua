use ulua_common::fflag;

use crate::records::type_arena::TypeArena;

pub fn unfreeze(arena: &mut TypeArena) {
  if !fflag::DebugLuauFreezeArena.get() {
    return;
  }

  arena.types.unfreeze();
  arena.type_packs.unfreeze();
}
