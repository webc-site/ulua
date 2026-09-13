use core::mem::zeroed;

use ulua_common::{FInt, records::dense_hash_set::DenseHashSet};

use crate::{
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id, is_prim::is_prim,
    is_string::is_string,
  },
  records::{
    any_type::AnyType, intersection_type::IntersectionType, metatable_type::MetatableType,
    primitive_type::PrimitiveType, recursion_limiter::RecursionLimiter, table_type::TableType,
    union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};
pub fn has_length(ty: TypeId, seen: &mut DenseHashSet<TypeId>, recursion_count: &mut i32) -> bool {
  let mut _rl = RecursionLimiter {
    base: unsafe { zeroed() },
    native_stack_guard: unsafe { zeroed() },
  };
  _rl.recursion_limiter_recursion_limiter(
    "Type::hasLength",
    recursion_count as *mut i32,
    FInt::LuauTypeInferRecursionLimit.get(),
  );

  let ty = follow_type_id(ty);

  if seen.contains(&ty) {
    return true;
  }

  if is_string(ty)
    || is_prim(ty, PrimitiveType::TABLE)
    || get_type_id::<AnyType>(ty).is_some()
    || get_type_id::<TableType>(ty).is_some()
    || get_type_id::<MetatableType>(ty).is_some()
  {
    return true;
  }

  if let Some(uty) = get_type_id::<UnionType>(ty) {
    seen.insert(ty);

    for &part in &uty.options {
      if !has_length(part, seen, recursion_count) {
        return false;
      }
    }

    return true;
  }

  if let Some(ity) = get_type_id::<IntersectionType>(ty) {
    seen.insert(ty);

    for &part in &ity.parts {
      if has_length(part, seen, recursion_count) {
        return true;
      }
    }

    return false;
  }

  false
}
