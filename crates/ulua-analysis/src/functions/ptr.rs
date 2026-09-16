//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TypeOrPack.cpp:9:ptr`
//! Source: `Analysis/src/TypeOrPack.cpp:9-17` (hand-ported)

/// C++ `const void* ptr(TypeOrPack tyOrTp)`.
use core::ffi::c_void;

use ulua_common::macros::luau_unreachable::LUAU_UNREACHABLE;

use crate::type_aliases::{
  type_id::TypeId,
  type_or_pack::{TypeOrPack, TypeOrPackMember},
  type_pack_id::TypePackId,
};
pub fn ptr(ty_or_tp: TypeOrPack) -> *const c_void {
  if let Some(ty) = TypeId::get_if(&ty_or_tp) {
    *ty as *const c_void
  } else if let Some(tp) = TypePackId::get_if(&ty_or_tp) {
    *tp as *const c_void
  } else {
    LUAU_UNREACHABLE!()
  }
}
