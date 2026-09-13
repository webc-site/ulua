use crate::{
  records::{
    builtin_types::BuiltinTypes, intersection_builder::IntersectionBuilder, type_arena::TypeArena,
  },
  type_aliases::type_id::TypeId,
};

pub fn add_intersection(
  arena: *mut TypeArena,
  builtin_types: *mut BuiltinTypes,
  list: &[TypeId],
) -> TypeId {
  let mut ib = IntersectionBuilder::new(arena, builtin_types);
  ib.reserve(list.len());
  for &part in list {
    ib.add(part);
  }
  ib.build()
}
