use crate::{
  records::{builtin_types::BuiltinTypes, type_arena::TypeArena, union_builder::UnionBuilder},
  type_aliases::type_id::TypeId,
};

pub fn add_union(
  arena: *mut TypeArena,
  builtin_types: *mut BuiltinTypes,
  list: &[TypeId],
) -> TypeId {
  let mut ub = UnionBuilder::new(arena, builtin_types);
  ub.reserve(list.len());
  for &option in list.iter() {
    ub.add(option);
  }
  ub.build()
}
