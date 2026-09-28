use crate::{
  functions::{begin_type::begin_union_type, follow_type, get_type, is_prim::is_prim_or_singleton},
  records::{
    boolean_singleton::BooleanSingleton, primitive_type::PrimitiveType, union_type::UnionType,
  },
  type_aliases::{singleton_variant::SingletonVariantMember, type_id::TypeId},
};

pub fn is_boolean(ty: TypeId) -> bool {
  if is_prim_or_singleton(ty, PrimitiveType::BOOLEAN, |variant| {
    BooleanSingleton::get_if(variant).is_some()
  }) {
    return true;
  }

  // C++ `std::all_of(begin(utv), end(utv), isBoolean)`——UnionTypeIterator
  // follow Bound 并展平嵌套 union,裸遍历 options 会漏掉嵌套成员。
  if let Some(utv) = get_type::get::<UnionType>(follow_type::follow(ty)) {
    return begin_union_type(utv).all(is_boolean);
  }

  false
}
