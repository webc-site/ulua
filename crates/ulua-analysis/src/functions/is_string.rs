use crate::{
  functions::{begin_type::begin_union_type, follow_type, get_type, is_prim::is_prim_or_singleton},
  records::{
    primitive_type::PrimitiveType, string_singleton::StringSingleton, union_type::UnionType,
  },
  type_aliases::{singleton_variant::SingletonVariantMember, type_id::TypeId},
};
pub fn is_string(ty: TypeId) -> bool {
  let ty = follow_type::follow(ty);

  if is_prim_or_singleton(ty, PrimitiveType::STRING, |variant| {
    StringSingleton::get_if(variant).is_some()
  }) {
    return true;
  }

  if let Some(utv) = get_type::get::<UnionType>(ty) {
    // C++ `std::all_of(begin(utv), end(utv), isString)` — UnionTypeIterator
    // 会 follow 并展平嵌套 union，且带 seen 集合跳过环。直接裸递归
    // `utv.options` 在结构自引用 union 上会栈溢出。
    return begin_union_type(utv).all(is_string);
  }

  false
}
