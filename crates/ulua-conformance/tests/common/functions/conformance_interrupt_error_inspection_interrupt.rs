use core::{ffi::c_int, sync::atomic::Ordering};

use ulua_vm::{macros::lua_l_error::luaL_error, records::lua_state::LuaState};

use crate::common::records::conformance_interrupt_error_inspection_state::CONFORMANCE_INTERRUPT_ERROR_INSPECTION_STATE;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_interrupt_error_inspection_interrupt(
  l: *mut LuaState,
  gc: c_int,
) {
  // 中断计数与目标都是 safe 原子量（SeqCst），无需 unsafe；GC 阶段直接放行。
  if gc >= 0 {
    return;
  }

  let step = CONFORMANCE_INTERRUPT_ERROR_INSPECTION_STATE
    .step
    .load(Ordering::SeqCst);
  let target = CONFORMANCE_INTERRUPT_ERROR_INSPECTION_STATE
    .target
    .load(Ordering::SeqCst);

  if step == target {
    // Safety: `l` 为本用例存活的 LuaState；按 cpp 在此抛 "test" Lua 错误，该调用不返回。
    unsafe { luaL_error!(l, "test") };
  }

  CONFORMANCE_INTERRUPT_ERROR_INSPECTION_STATE
    .step
    .store(step + 1, Ordering::SeqCst);
}
