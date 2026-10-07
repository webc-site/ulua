use crate::{
  functions::{follow_type, get_type, try_strip_union_from_nil::try_strip_union_from_nil},
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, type_arena::TypeArena, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

/// 形参全部为受检类型（[`Handle`] + `&mut TypeArena` + 值），C++
/// `stripNil` 的非空契约已由类型编码，无需 unsafe。
pub fn strip_nil(builtin_types: Handle<BuiltinTypes>, arena: &mut TypeArena, ty: TypeId) -> TypeId {
  let builtin_types = builtin_types.get();
  let ty = follow_type::follow(ty);

  if get_type::get::<UnionType>(ty).is_none() {
    return follow_type::follow(ty);
  }

  let cleaned = try_strip_union_from_nil(arena, ty);

  // 双写合一：`is_none()` 早退与 `unwrap()` 并为 let-else，Some 直接绑定。
  // If there is no union option without 'nil'
  let Some(cleaned) = cleaned else {
    return builtin_types.nil_type;
  };

  follow_type::follow(cleaned)
}
