use alloc::vec::Vec;

use crate::{
  records::{intersection_type::IntersectionType, type_arena::TypeArena},
  type_aliases::type_id::TypeId,
};

pub fn make_intersection(arena: &mut TypeArena, types: Vec<TypeId>) -> TypeId {
  arena.add_type(IntersectionType { parts: types })
}
