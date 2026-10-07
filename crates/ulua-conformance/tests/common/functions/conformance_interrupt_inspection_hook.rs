use ulua_vm::records::{lua_debug::LuaDebug, lua_state::LuaState};

use crate::common::functions::safe_api::getinfo;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_interrupt_inspection_hook(
  l: *mut LuaState,
  ar: *mut LuaDebug,
) {
  // `ar` 由 VM 在钩子点交回，指向本回调期间存活的 LuaDebug 记录；`nsl` 掩码
  // 只读当前帧信息，不改动栈。
  // Safety: `ar` 为 VM 传入的存活记录（C ABI 钩子契约）。
  assert_ne!(0, getinfo(l, 0, b"nsl\0", unsafe { &mut *ar }));
}
