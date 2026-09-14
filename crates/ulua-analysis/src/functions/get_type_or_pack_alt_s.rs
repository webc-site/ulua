//! Source: `Analysis/include/Luau/TypeOrPack.h:30-37` (hand-ported)
/// C++ `get<T>(const TypeOrPack&)` for T a member of TypePackVariant: unwrap
/// the TypePackId member, then `get<T>(*tp)`.
use core::ptr::null;

use crate::{
  functions::get_type_pack::get,
  type_aliases::{
    type_or_pack::{TypeOrPack, TypeOrPackMember},
    type_pack_id::TypePackId,
    type_pack_variant::TypePackVariantMember,
  },
};
pub fn get_type_or_pack_mut_2<T: TypePackVariantMember + 'static>(
  ty_or_tp: &TypeOrPack,
) -> *const T {
  match TypePackId::get_if(ty_or_tp) {
    Some(tp) => get::<T>(*tp).map_or(null(), |r| r as *const T),
    None => null(),
  }
}
