use alloc::vec::Vec;

use crate::{
  records::{type_arena::TypeArena, union_type::UnionType},
  type_aliases::type_id::TypeId,
};

pub fn make_union(arena: &mut TypeArena, types: Vec<TypeId>) -> TypeId {
  arena.add_type(UnionType { options: types })
}
