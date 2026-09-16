use ulua_common::records::variant::Variant2;

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    boolean_singleton::BooleanSingleton, primitive_type::PrimitiveType,
    singleton_type::SingletonType, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};

pub fn is_approximately_falsy_type(ty: TypeId) -> bool {
  let ty = follow_type_id(ty);

  let mut seen_nil = false;
  let mut seen_false = false;

  if let Some(utv) = get_type_id::<UnionType>(ty).as_ref() {
    for option in utv.options.iter() {
      let option = follow_type_id(*option);

      if let Some(ptv) = get_type_id::<PrimitiveType>(option).as_ref() {
        if ptv.r#type == PrimitiveType::NIL_TYPE {
          seen_nil = true;
        } else {
          return false;
        }
      } else if let Some(stv) = get_type_id::<SingletonType>(option).as_ref() {
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
