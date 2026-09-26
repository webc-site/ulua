//! Source: `Analysis/src/TypeOrPack.cpp:9-17` (hand-ported)

// C++ `const void* ptr(TypeOrPack tyOrTp)`.
use ulua_common::macros::luau_assert::LUAU_UNREACHABLE;

use crate::type_aliases::{
  type_id::TypeId,
  type_or_pack::{TypeOrPack, TypeOrPackMember},
  type_pack_id::TypePackId,
};
pub fn ptr(ty_or_tp: TypeOrPack) -> *const () {
  if let Some(ty) = TypeId::get_if(&ty_or_tp) {
    *ty as *const ()
  } else if let Some(tp) = TypePackId::get_if(&ty_or_tp) {
    *tp as *const ()
  } else {
    LUAU_UNREACHABLE!()
  }
}
