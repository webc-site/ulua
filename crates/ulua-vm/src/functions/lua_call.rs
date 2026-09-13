//! Node: `cxx:Function:Luau.VM:VM/src/lapi.cpp:1082:lua_call`
//! Source: `VM/src/lapi.cpp:1082-1092` (hand-ported)

use core::ffi::c_int;

use crate::{
  functions::lua_d_call::lua_d_call,
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
    api_check!(
      l,
      nresults == LUA_MULTRET
        || (*(*l).ci).top.offset_from((*l).top) >= (nresults - nargs) as isize
    );

    let func: StkId = (*l).top.sub((nargs + 1) as usize);
    lua_d_call(l, func, nresults);

    if nresults == LUA_MULTRET && (*l).top >= (*(*l).ci).top {
      (*(*l).ci).top = (*l).top;
    }
  }
}
