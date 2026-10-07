use core::sync::atomic::Ordering;

use ulua_vm::records::lua_state::LuaState;

use crate::common::{
  functions::{
    conformance_debugger_breakpoint::conformance_debugger_breakpoint,
    conformance_debugger_debug_break::conformance_debugger_debug_break,
    conformance_debugger_debug_interrupt::conformance_debugger_debug_interrupt,
    conformance_debugger_debug_step::conformance_debugger_debug_step,
    safe_api::{callbacks_mut, pushcclosurek, singlestep, state_mut},
  },
  records::conformance_debugger_state::CONFORMANCE_DEBUGGER_STATE,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_debugger_setup(l: *mut LuaState) {
  // 单步开关取本用例的原子状态位（只写 0/1）。
  singlestep(
    l,
    CONFORMANCE_DEBUGGER_STATE.singlestep.load(Ordering::SeqCst),
  );

  // 三个调试钩子都是 `extern "C-unwind"` 桩，由 VM 在调试点回调。
  let cb = callbacks_mut(l);
  cb.debugstep = Some(conformance_debugger_debug_step);
  cb.debugbreak = Some(conformance_debugger_debug_break);
  cb.debuginterrupt = Some(conformance_debugger_debug_interrupt);

  // 注册 breakpoint 闭包并登记为全局（名字为 NUL 结尾静态串）。
  pushcclosurek(
    l,
    Some(conformance_debugger_breakpoint),
    Some(b"breakpoint\0"),
    0,
    None,
  );
  state_mut(l).set_global_str("breakpoint");
}
