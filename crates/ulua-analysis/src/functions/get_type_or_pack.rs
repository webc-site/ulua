//! Source: `Analysis/include/Luau/TypeOrPack.h:15-19` (hand-ported)
/// C++ `get<T>(const TypeOrPack&)` for T a DIRECT member (TypeId/TypePackId).
/// (Signature-pinned overload name; the TypeVariant / TypePackVariant
/// overloads are `get_type_or_pack` / `get_type_or_pack_mut_2`.)
use core::ptr::null;

use crate::type_aliases::type_or_pack::{TypeOrPack, TypeOrPackMember};
pub fn get_type_or_pack_mut<T: TypeOrPackMember>(ty_or_tp: &TypeOrPack) -> *const T {
  match T::get_if(ty_or_tp) {
    Some(r) => r as *const T,
    None => null(),
  }
}
