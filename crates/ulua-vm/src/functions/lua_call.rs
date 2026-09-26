//! Source: `VM/src/lapi.cpp:1082-1092` (hand-ported)

use crate::{
  functions::{ensure_stack::ensure_stack, lua_d_call::lua_d_call},
  macros::{api_check::api_check, api_checknelems::api_checknelems, lua_multret::LUA_MULTRET},
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且 `(*l).status==0`；栈顶须有 `nargs+1` 个元素（`api_checknelems`），
/// 其下标 `top-(nargs+1)` 处为可调用函数 TValue、其后为参数；`nargs>=0`、`nresults>=LUA_MULTRET`；
/// `nresults>LUA_MULTRET` 时结果须为 `nresults` 个槽，`luaD_call` 可 GC/抛错，需受保护帧。cpp `lapi.cpp:1082`。
pub unsafe fn lua_call(l: *mut LuaState, nargs: i32, nresults: i32) {
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
