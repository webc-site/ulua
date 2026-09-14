//! Node: `cxx:Function:Luau.VM:VM/src/ldebug.cpp:185:lua_getinfo`
//!
//! `lua_getinfo` — resolve a stack `level` (negative = relative to top, else a
//! call-info depth) to its closure, fill `ar` via `auxgetinfo`, and (when the
//! `f` option pushed the function) place it on the stack. Returns 1 if a
//! function was found at that level, else 0.

use core::{
  ffi::{c_char, c_int},
  ptr::null_mut,
};

use ulua_common::LUAU_ASSERT;

use crate::{
  functions::auxgetinfo::auxgetinfo,
  macros::{
    clvalue::clvalue, incr_top::incr_top, lua_c_threadbarrier::luaC_threadbarrier,
    setclvalue::setclvalue, ttisfunction::ttisfunction,
  },
  records::{call_info::CallInfo, closure::Closure, lua_debug::LuaDebug},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_getinfo(
  l: *mut lua_State,
  level: c_int,
  what: *const c_char,
  ar: *mut LuaDebug,
) -> c_int {
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
      if !ttisfunction!(func) {
        return 0;
      }

      f = clvalue!(func);
    } else if (level as u32) < (*l).ci.offset_from((*l).base_ci) as u32 {
      ci = (*l).ci.offset(-(level as isize));
      LUAU_ASSERT!(ttisfunction!((*ci).func));
      f = clvalue!((*ci).func);
    }

    if !f.is_null() {
      // auxgetinfo fills ar and optionally requests to put closure on stack
      let fcl = auxgetinfo(l, what, ar, f, ci);
      if !fcl.is_null() {
        luaC_threadbarrier!(l);
        setclvalue!(l, (*l).top, fcl);
        incr_top!(l);
      }
    }

    if f.is_null() { 0 } else { 1 }
  }
}
