use crate::{
  functions::{follow_type, get_type, is_prim::is_nil},
  records::{any_type::AnyType, union_type::UnionType, unknown_type::UnknownType},
  type_aliases::{collections::HashSet, type_id::TypeId},
};

pub fn is_optional(ty: TypeId) -> bool {
  let mut seen = HashSet::<TypeId>::new();
  let mut stack = vec![ty];

  while let Some(ty) = stack.pop() {
    let ty = follow_type::follow(ty);
    if !seen.insert(ty) {
      continue;
    }

    if is_nil(ty)
      || get_type::get::<AnyType>(ty).is_some()
      || get_type::get::<UnknownType>(ty).is_some()
    {
      return true;
    }

    if let Some(utv) = get_type::get::<UnionType>(ty) {
      stack.extend(utv.options.iter().copied());
    }
  }

  false
}
