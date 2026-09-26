use ulua_vm::records::{lua_debug::LuaDebug, lua_state::LuaState};

use crate::common::records::conformance_debugger_state::CONFORMANCE_DEBUGGER_STATE;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_debugger_debug_interrupt(
  _l: *mut LuaState,
  ar: *mut LuaDebug,
) {
  // 幂等前置检查只读 safe 原子量：中断线程此时必须尚未登记。
  assert!(
    CONFORMANCE_DEBUGGER_STATE
      .take_interruptedthread()
      .is_none()
  );
  // Safety: `ar` 由 VM 在中断点交回，指向本回调期间存活的 LuaDebug 记录。
  let userdata = unsafe { (*ar).userdata };
  assert!(!userdata.is_null());

  // 登记为 safe 原子指针存储：值即 VM 写入该调试记录的用户数据指针（被中断线程状态）。
  CONFORMANCE_DEBUGGER_STATE.set_interruptedthread(Some(userdata as *mut LuaState));
}
