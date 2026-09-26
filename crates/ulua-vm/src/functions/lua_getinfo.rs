//! Source: `VM/src/ldebug.cpp:185`
//!
//! `lua_getinfo` — resolve a stack `level` (negative = relative to top, else a
//! call-info depth) to its closure, fill `ar` via `auxgetinfo`, and (when the
//! `f` option pushed the function) place it on the stack. Returns 1 if a
//! function was found at that level, else 0.

use core::{ffi::c_char, ptr::null_mut};

use ulua_common::LUAU_ASSERT;

use crate::{
  functions::{auxgetinfo::auxgetinfo, lapi_barrier::lua_c_threadbarrier_lapi},
  macros::{incr_top::incr_top, setclvalue::setclvalue},
  records::{call_info::CallInfo, closure::Closure, lua_debug::LuaDebug, lua_state::LuaState},
};

/// # Safety
/// `l` 必须指向存活 `LuaState` 且其栈/调用帧深度覆盖 `level`（对 top/base_ci 做裸 offset 游走）；`what` 须为
/// 可读的 NUL 结尾 C 串；`ar` 须为存活可写 LuaDebug（auxgetinfo 直填字段）；`f` 选项命中时会向 `l` 压栈一个
/// 闭包，调用方须预留栈顶余量。对应 cpp ldebug.cpp:185。
pub unsafe fn lua_getinfo(
  l: *mut LuaState,
  level: i32,
  what: *const c_char,
  ar: *mut LuaDebug,
) -> i32 {
  // Safety: 契约保证 `L` 调用栈深度覆盖 level、`what` 为可读 C 串且 `ar` 可写；分支检查后 auxgetinfo 前置成立，出错路径不泄漏栈槽
  unsafe {
    let mut f: *mut Closure = null_mut();
    let mut ci: *mut CallInfo = null_mut();

    if level < 0 {
      // element has to be within stack
      if (-level) as isize > (*l).top.offset_from((*l).base) {
        return 0;
      }

      let func = (*l).top.offset(level as isize);

      // and it has to be a function
      if !(*func).is_function() {
        return 0;
      }

      f = (*func).as_closure_ptr();
    } else if (level as u32) < (*l).ci.offset_from((*l).base_ci) as u32 {
      ci = (*l).ci.offset(-(level as isize));
      LUAU_ASSERT!((*(*ci).func).is_function());
      f = (*(*ci).func).as_closure_ptr();
    }

    if !f.is_null() {
      // auxgetinfo fills ar and optionally requests to put closure on stack
      let fcl = auxgetinfo(l, what, ar, f, ci);
      if !fcl.is_null() {
        lua_c_threadbarrier_lapi(l);
        setclvalue!(l, (*l).top, fcl);
        incr_top!(l);
      }
    }

    if f.is_null() { 0 } else { 1 }
  }
}
