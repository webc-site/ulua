use crate::{
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
    try_strip_union_from_nil::try_strip_union_from_nil,
  },
  records::{builtin_types::BuiltinTypes, type_arena::TypeArena, union_type::UnionType},
  type_aliases::type_id::TypeId,
};

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn strip_nil(
  builtin_types: *mut BuiltinTypes,
  arena: &mut TypeArena,
  ty: TypeId,
) -> TypeId {
  let builtin_types = unsafe { builtin_types.as_ref().expect("builtin_types is null") };
  let ty = follow_type_id(ty);

  if get_type_id::<UnionType>(ty).is_none() {
    return follow_type_id(ty);
  }

  let cleaned = try_strip_union_from_nil(arena, ty);

  // If there is no union option without 'nil'
  if cleaned.is_none() {
    return builtin_types.nil_type;
  }

  follow_type_id(cleaned.unwrap())
}
