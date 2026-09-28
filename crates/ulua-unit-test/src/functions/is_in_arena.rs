use ulua_analysis::{records::type_arena::TypeArena, type_aliases::type_id::TypeId};

pub fn is_in_arena(t: TypeId, arena: &TypeArena) -> bool {
  arena.types.contains(t)
}
