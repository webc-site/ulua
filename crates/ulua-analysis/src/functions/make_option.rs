use core::ptr::NonNull;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::make_union::make_union,
  records::{arena_handle::alias_ref, builtin_types::BuiltinTypes, type_arena::TypeArena},
  type_aliases::type_id::TypeId,
};

pub fn make_option(
  builtin_types: NonNull<BuiltinTypes>,
  arena: &mut TypeArena,
  t: TypeId,
) -> TypeId {
  LUAU_ASSERT!(!t.is_null());
  let builtin_types = alias_ref(builtin_types.as_ptr());
  make_union(arena, vec![builtin_types.nil_type, t])
}
