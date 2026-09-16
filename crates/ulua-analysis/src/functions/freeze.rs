use ulua_common::FFlag;

use crate::records::type_arena::TypeArena;

/// C++ `void freeze(TypeArena& arena)` (TypeArena.cpp:138-145). Freezing is a
/// no-op unless `DebugLuauFreezeArena` is set — the underlying `pagedFreeze`
/// asserts the flag, so the guard must be here.
pub fn freeze(arena: &mut TypeArena) {
  if !FFlag::DebugLuauFreezeArena.get() {
    return;
  }

  arena.types.freeze();
  arena.type_packs.freeze();
}
