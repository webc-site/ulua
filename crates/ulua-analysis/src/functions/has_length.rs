use ulua_common::{fint, records::dense_hash_set::DenseHashSet};

use crate::{
  functions::{follow_type, get_type, is_prim::is_prim, is_string::is_string},
  records::{
    any_type::AnyType, intersection_type::IntersectionType, metatable_type::MetatableType,
    primitive_type::PrimitiveType, recursion_limiter::RecursionLimiter, table_type::TableType,
    union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};
pub fn has_length(ty: TypeId, seen: &mut DenseHashSet<TypeId>, recursion_count: &mut i32) -> bool {
  let _rl = RecursionLimiter::new(
    "Type::hasLength",
    recursion_count,
    fint::LuauTypeInferRecursionLimit.get(),
  );

  let ty = follow_type::follow(ty);

  if seen.contains(&ty) {
    return true;
  }

  if is_string(ty)
    || is_prim(ty, PrimitiveType::TABLE)
    || get_type::get::<AnyType>(ty).is_some()
    || get_type::get::<TableType>(ty).is_some()
    || get_type::get::<MetatableType>(ty).is_some()
  {
    return true;
  }

  if let Some(uty) = get_type::get::<UnionType>(ty) {
    seen.insert(ty);

    for &part in &uty.options {
      if !has_length(part, seen, recursion_count) {
        return false;
      }
    }

    return true;
  }

  if let Some(ity) = get_type::get::<IntersectionType>(ty) {
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
