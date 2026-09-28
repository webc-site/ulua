use crate::{
  functions::{follow_type, get_type},
  records::{
    primitive_type::{PrimitiveType, Type},
    singleton_type::SingletonType,
  },
  type_aliases::{singleton_variant::SingletonVariant, type_id::TypeId},
};

pub fn is_prim(ty: TypeId, prim_type: Type) -> bool {
  let followed = follow_type::follow(ty);
  get_type::get::<PrimitiveType>(followed).is_some_and(|p| p.r#type == prim_type)
}

/// `isString`/`isBoolean`（Type.cpp）共享的前两段骨架：follow 后先判 primitive，
/// 再按 `variant_pred` 判 SingletonType 变体；union 递归段由调用方各自保留
/// （`follow` 幂等，重复 follow 不改结果）。
pub fn is_prim_or_singleton(
  ty: TypeId,
  prim_type: Type,
  variant_pred: impl Fn(&SingletonVariant) -> bool,
) -> bool {
  let followed = follow_type::follow(ty);

  if get_type::get::<PrimitiveType>(followed).is_some_and(|p| p.r#type == prim_type) {
    return true;
  }

  if let Some(st) = get_type::get::<SingletonType>(followed)
    && variant_pred(&st.variant)
  {
    return true;
  }

  false
}

#[inline]
pub fn is_nil(ty: TypeId) -> bool {
  is_prim(ty, PrimitiveType::NIL_TYPE)
}

#[inline]
pub fn is_number(ty: TypeId) -> bool {
  is_prim(ty, PrimitiveType::NUMBER)
}

#[inline]
pub fn is_integer(ty: TypeId) -> bool {
  is_prim(ty, PrimitiveType::INTEGER)
}

#[inline]
pub fn is_buffer(ty: TypeId) -> bool {
  is_prim(ty, PrimitiveType::BUFFER)
}

#[inline]
pub fn is_thread(ty: TypeId) -> bool {
  is_prim(ty, PrimitiveType::THREAD)
}
