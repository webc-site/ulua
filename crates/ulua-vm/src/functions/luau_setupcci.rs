//! Node: `cxx:Function:Luau.VM:VM/src/lvmexecute.cpp:206:luau_setupcci`
//!
//! Push and initialize a fresh `CallInfo` for a C continuation call: point it at
//! `fun`, give it a `LUA_MINSTACK` window, clear the saved pc/flags, bump the
//! closure usage counter (when enabled), and ensure the stack has room.

use core::{ffi::c_int, ptr::null};

use ulua_common::{FFlag, LUAU_ASSERT};

use crate::{
  macros::{
    clvalue::clvalue, incr_ci::incr_ci, lua_d_checkstackfornewci::luaD_checkstackfornewci,
    lua_minstack::LUA_MINSTACK, ttisfunction::ttisfunction,
  },
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn luau_setupcci(l: *mut lua_State, nresults: c_int, fun: StkId) {
  unsafe {
    let ci = incr_ci!(l);

    (*ci).func = fun;
    (*ci).base = fun.add(1);
    (*ci).top = (*l).top.add(LUA_MINSTACK as usize);
    (*ci).savedpc = null();
    (*ci).flags = 0;
    (*ci).nresults = nresults;

    if FFlag::LuauClosureUsageCounter.get() {
      (*clvalue!(fun)).usage += 1;
    }

    (*l).base = fun.add(1);

    luaD_checkstackfornewci(l, LUA_MINSTACK);

    LUAU_ASSERT!((*ci).top <= (*l).stack_last);
    LUAU_ASSERT!(ttisfunction!((*ci).func));
  }
}
