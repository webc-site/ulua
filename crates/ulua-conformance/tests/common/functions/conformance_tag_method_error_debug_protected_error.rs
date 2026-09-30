use core::sync::atomic::Ordering;

use ulua_vm::{
  functions::{lua_break::lua_break, lua_isyieldable::lua_isyieldable},
  records::lua_state::LuaState,
};

use crate::common::{
  functions::get_first_luau_frame_debug_info::get_first_luau_frame_debug_info,
  records::conformance_tag_method_error_state::CONFORMANCE_TAG_METHOD_ERROR_STATE,
};

const EXPECTED_HITS: [i32; 3] = [37, 54, 73];
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_tag_method_error_debug_protected_error(
  l: *mut LuaState,
) {
  // Safety: `l` 为本用例存活的 LuaState；取首个 Luau 帧的调试信息（无帧时 None）。
  let ar = unsafe { get_first_luau_frame_debug_info(l) };

  // Safety: `l` 存活；受保护错误回调按 cpp 契约总在可 yield 的上下文中触发。
  assert_ne!(unsafe { lua_isyieldable(l) }, 0);
  let ar = ar.expect("expected a Lua frame");

  // 命中序号与行号断言只读写 safe 原子量，不需要 unsafe。
  let index = CONFORMANCE_TAG_METHOD_ERROR_STATE
    .index
    .fetch_add(1, Ordering::SeqCst);
  assert!((index as usize) < EXPECTED_HITS.len());
  assert_eq!(ar.currentline, EXPECTED_HITS[index as usize]);

  if CONFORMANCE_TAG_METHOD_ERROR_STATE
    .lua_break
    .load(Ordering::SeqCst)
  {
    // Safety: `l` 存活；按用例开关请求在下一个安全点打断执行。
    unsafe { lua_break(l) };
  }
}
