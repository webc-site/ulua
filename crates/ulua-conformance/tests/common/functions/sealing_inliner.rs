// 边界契约测试：allocator FFI 返回 null 系分配失败契约（既有约定 review.md §2）
//! Source: `tests/FeedbackVector.test.cpp`

use core::ptr::null_mut;

use ulua_vm::records::{closure::Closure, lua_state::LuaState, proto::Proto};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn sealing_inliner(
  _l: *mut LuaState,
  _caller: *mut Closure,
  _target: *mut Closure,
  _pc: u32,
) -> *mut Proto {
  // FFI: c-API 要求 NULL
  null_mut()
}
