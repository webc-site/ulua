//! Node: `cxx:Function:Luau.Analysis:Analysis/src/Type.cpp:61:follow`
//! Source: `Analysis/src/Type.cpp` (Type.cpp:61-72, hand-ported)

use core::{ffi::c_void, ptr::null};

use crate::{
  enums::follow_option::FollowOption, functions::follow_type_alt_e::follow_full,
  type_aliases::type_id::TypeId,
};
fn identity_mapper(_context: *const c_void, t: TypeId) -> TypeId {
  t
}

/// # Safety
/// 调用方须保证满足 C++ 原实现定义的内部不变量。
pub unsafe fn follow_with_option(t: TypeId, follow_option: FollowOption) -> TypeId {
  // SAFETY: t 的有效性由调用方按 C++ 同契约保证；unsafe 由 follow_full 承担。
  follow_full(t, follow_option, null(), identity_mapper)
}

pub use follow_with_option as follow_type_id_follow_option;
