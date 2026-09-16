//! Node: `cxx:Function:Luau.Analysis:Analysis/src/TypePack.cpp:245:follow`
//! Source: `Analysis/src/TypePack.cpp` (TypePack.cpp:245-255, hand-ported)

use core::{ffi::c_void, ptr::null};

use crate::{
  functions::follow_type_pack_alt_h::follow_pack_full, type_aliases::type_pack_id::TypePackId,
};
fn identity_mapper(_context: *const c_void, t: TypePackId) -> TypePackId {
  t
}

/// # Safety
/// 调用方须保证满足 C++ 原实现定义的内部不变量。
/// C++ `TypePackId follow(TypePackId tp)`：安全封装，不变量与 C++ 调用方契约一致。
pub unsafe fn follow(tp: TypePackId) -> TypePackId {
  // SAFETY: tp 的有效性由调用方按 C++ 同契约保证。
  unsafe { follow_pack_full(tp, null(), identity_mapper) }
}

pub use follow as follow_type_pack_id;
