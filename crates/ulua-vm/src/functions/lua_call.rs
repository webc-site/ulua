//! Source: `VM/src/lapi.cpp:1082-1092` (hand-ported)

use core::ffi::c_int;

use crate::{
  functions::{ensure_stack::ensure_stack, lua_d_call::lua_d_call},
  macros::{api_check::api_check, api_checknelems::api_checknelems, lua_multret::LUA_MULTRET},
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_call(l: *mut lua_State, nargs: c_int, nresults: c_int) {
  unsafe {
    api_check!(l, nargs >= 0);
    api_check!(l, nresults >= LUA_MULTRET);
    api_checknelems!(l, nargs + 1);
    api_check!(l, (*l).status == 0);

    // cpp `ensure_stack(L, nresults - (nargs + 1))`：luaD_call 会按 nresults
    // 写结果槽，帧内空槽不够时必须先扩容。
    if nresults > nargs + 1 {
      ensure_stack(l, nresults - (nargs + 1));
    }

    let func: StkId = (*l).top.sub((nargs + 1) as usize);
    lua_d_call(l, func, nresults);

    if nresults == LUA_MULTRET && (*l).top >= (*(*l).ci).top {
      (*(*l).ci).top = (*l).top;
    }
  }
}
