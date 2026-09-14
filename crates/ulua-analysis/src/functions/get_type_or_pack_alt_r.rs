//! Source: `Analysis/include/Luau/TypeOrPack.h:21-28` (hand-ported)
/// C++ `get<T>(const TypeOrPack&)` for T a member of TypeVariant: unwrap the
/// TypeId member, then `get<T>(*ty)`.
use core::ptr::null;

use crate::{
  functions::get_type_alt_j::get,
  type_aliases::{
    type_id::TypeId,
    type_or_pack::{TypeOrPack, TypeOrPackMember},
    type_variant::TypeVariantMember,
  },
};
pub fn get_type_or_pack<T: TypeVariantMember + 'static>(ty_or_tp: &TypeOrPack) -> *const T {
  match TypeId::get_if(ty_or_tp) {
    Some(ty) => get::<T>(*ty).map_or(null(), |r| r as *const T),
    None => null(),
  }
}
