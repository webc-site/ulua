//! Source: `VM/src/lgc.cpp` (lgc.cpp:485-498, hand-ported)

use core::{ffi::c_void, ptr::null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{lua_d_rawrunprotected_ldo::lua_d_rawrunprotected_mut, shrinkstack::shrinkstack},
  records::lua_state::LuaState,
};

/// # Safety
/// 仅作 lua_d_rawrunprotected_mut 的受保护回调：`l` 为该受保护状态本体；shrinkstack 内 realloc
/// 失败抛 ErrMem 由保护帧吞掉，禁止在受保护帧之外直接调用。对应 cpp lgc.cpp:525 `CallContext::run`
// C++ uses a local `struct CallContext { static void run(...) }`; a local fn is
// the Rust equivalent of that protected-call trampoline.
unsafe extern "C-unwind" fn run(l: *mut LuaState, _ud: *mut c_void) {
  // Safety: 契约保证 `l` 存活；缩栈在受保护回调中执行，失败仅保持原栈大小
  unsafe {
    shrinkstack(l);
  }
}

/// # Safety
/// 调用方须保证：`l` 存活且 CallInfo 数组尚余至少一层供 rawrunprotected 开保护帧；本函数只在
/// GC propagate 阶段对已灰化/黑化的存活线程调用，返回后栈可能因 ErrMem 保持原大小，调用方
/// 不得假设 stacksize 已缩小。cpp lgc.cpp:522 `shrinkstackprotected`
pub(crate) unsafe fn shrinkstackprotected(l: *mut LuaState) {
  unsafe {
    // the resize call can fail on exception, in which case we will continue with original size
    let status = lua_d_rawrunprotected_mut(l, Some(run), null_mut());
    LUAU_ASSERT!(status == LuaStatus::Ok as i32 || status == LuaStatus::ErrMem as i32);
  }
}
