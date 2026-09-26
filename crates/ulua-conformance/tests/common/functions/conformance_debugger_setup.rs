use core::sync::atomic::Ordering;

use ulua_vm::{
  functions::{
    lua_callbacks::lua_callbacks, lua_pushcclosurek::lua_pushcclosurek,
    lua_singlestep::lua_singlestep,
  },
  macros::lua_setglobal::lua_setglobal,
  records::lua_state::LuaState,
};

use crate::common::{
  functions::{
    conformance_debugger_breakpoint::conformance_debugger_breakpoint,
    conformance_debugger_debug_break::conformance_debugger_debug_break,
    conformance_debugger_debug_interrupt::conformance_debugger_debug_interrupt,
    conformance_debugger_debug_step::conformance_debugger_debug_step, cstr::cstr,
  },
  records::conformance_debugger_state::CONFORMANCE_DEBUGGER_STATE,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_debugger_setup(l: *mut LuaState) {
  // Safety: `l` 为本用例存活的 LuaState；`lua_callbacks` 返回该状态持有的回调表指针。
  let cb = unsafe { lua_callbacks(l) };

  // Safety: `l` 存活；单步开关取本用例的原子状态位（只写 0/1）。
  unsafe {
    lua_singlestep(
      &mut *l,
      if CONFORMANCE_DEBUGGER_STATE.singlestep.load(Ordering::SeqCst) {
        1
      } else {
        0
      },
    )
  };

  // Safety: `cb` 指向上一步取得的回调表（随 `l` 存活）；三个调试钩子都是
  // `extern "C-unwind"` 桩，由 VM 在调试点回调。
  unsafe {
    (*cb).debugstep = Some(conformance_debugger_debug_step);
    (*cb).debugbreak = Some(conformance_debugger_debug_break);
    (*cb).debuginterrupt = Some(conformance_debugger_debug_interrupt);
  }

  // Safety: `l` 存活；注册 breakpoint 闭包并登记为全局（名字为 NUL 结尾静态串）。
  unsafe {
    lua_pushcclosurek(
      l,
      Some(conformance_debugger_breakpoint),
      cstr(b"breakpoint\0"),
      0,
      None,
    );
    lua_setglobal(l, cstr(b"breakpoint\0"));
  }
}
