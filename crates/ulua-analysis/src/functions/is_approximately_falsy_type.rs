use ulua_common::records::variant::Variant2;

use crate::{
  functions::{begin_type::begin_union_type, follow_type, get_type},
  records::{
    boolean_singleton::BooleanSingleton, primitive_type::PrimitiveType,
    singleton_type::SingletonType, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

pub fn is_approximately_falsy_type(ty: TypeId) -> bool {
  let ty = follow_type::follow(ty);

  let mut seen_nil = false;
  let mut seen_false = false;

  if let Some(utv) = get_type::get::<UnionType>(ty).as_ref() {
    // C++ `for (auto option : ut)`——UnionTypeIterator 展平嵌套 union 并
    // follow,裸遍历 options 会漏掉嵌套成员。
    for option in begin_union_type(utv) {
      let option = follow_type::follow(option);

      if let Some(ptv) = get_type::get::<PrimitiveType>(option).as_ref() {
        if ptv.r#type == PrimitiveType::NIL_TYPE {
          seen_nil = true;
        } else {
          return false;
        }
      } else if let Some(stv) = get_type::get::<SingletonType>(option).as_ref() {
        if stv.variant == Variant2::V0(BooleanSingleton::new(false)) {
          seen_false = true;
        } else {
          return false;
        }
      } else {
        return false;
      }
    }
  } else {
    return false;
  }

  seen_false && seen_nil
}
