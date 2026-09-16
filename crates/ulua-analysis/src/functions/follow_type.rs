//! Node: `cxx:Function:Luau.Analysis:Analysis/src/Type.cpp:56:follow`
//! Source: `Analysis/src/Type.cpp` (Type.cpp:56-59, hand-ported)

use core::{ffi::c_void, ptr::null};

use crate::{
  enums::follow_option::FollowOption, functions::follow_type_alt_e::follow_full,
  type_aliases::type_id::TypeId,
};
fn identity_mapper(_context: *const c_void, t: TypeId) -> TypeId {
  t
}

/// C++ `TypeId follow(TypeId t)`：解引用 Bound/Table/Lazy 链。安全封装，
/// 不变量（t 有效）与 C++ 调用方契约一致，由 follow_full 内部 unsafe 承担。
pub fn follow(t: TypeId) -> TypeId {
  // SAFETY: t 的有效性由调用方按 C++ 同契约保证；unsafe 由 follow_full 承担。
  follow_full(t, FollowOption::Normal, null(), identity_mapper)
}

pub use crate::functions::follow_type::follow as follow_type_id;
