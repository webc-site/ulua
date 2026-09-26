use crate::{
  functions::{begin_type::begin_union_type, follow_type, get_type, is_prim::is_prim},
  records::{any_type::AnyType, primitive_type::PrimitiveType, union_type::UnionType},
  type_aliases::type_id::TypeId,
};

pub fn maybe_string(ty: TypeId) -> bool {
  let ty = follow_type::follow(ty);

  if is_prim(ty, PrimitiveType::STRING) || get_type::get::<AnyType>(ty).is_some() {
    return true;
  }

  // C++ `std::any_of(begin(utv), end(utv), maybeString)`——迭代器展平嵌套
  // union 并 follow,裸遍历 options 会漏掉嵌套成员。
  if let Some(utv) = get_type::get::<UnionType>(ty) {
    return begin_union_type(utv).any(maybe_string);
  }

  false
}
