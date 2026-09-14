use std::collections::HashSet;

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id, is_nil::is_nil},
  records::{any_type::AnyType, union_type::UnionType, unknown_type::UnknownType},
  type_aliases::type_id::TypeId,
};

pub fn is_optional(ty: TypeId) -> bool {
  let mut seen = HashSet::<TypeId>::new();
  let mut stack = vec![ty];

  while let Some(ty) = stack.pop() {
    let ty = follow_type_id(ty);
    if !seen.insert(ty) {
      continue;
    }

    if is_nil(ty)
      || get_type_id::<AnyType>(ty).is_some()
      || get_type_id::<UnknownType>(ty).is_some()
    {
      return true;
    }

    if let Some(utv) = get_type_id::<UnionType>(ty) {
      stack.extend(utv.options.iter().copied());
    }
  }

  false
}
