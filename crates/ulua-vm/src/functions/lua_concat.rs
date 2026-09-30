use crate::{
  functions::{
    ensure_stack::ensure_stack, lapi_barrier::lua_c_threadbarrier_lapi,
    lua_s_newlstr::lua_s_newlstr, lua_v_concat::lua_v_concat,
  },
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, api_incr_top::api_incr_top,
    lua_c_check_gc::lua_c_check_gc, setsvalue::setsvalue,
  },
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且 `(*l).top` 之下、`(*l).base` 之上至少留有 `n` 个
/// 待拼接 TValue（`api_checknelems` 校验 `top-base>=n`），`n` 不得为负（`api_check n>=0`）；
/// `n>=2` 会触发 `lua_c_check_gc` 与 `luaV_concat`（可分配、可抛错），`n==0` 会推入空串，
/// 均须在受保护帧内调用。cpp `lapi.cpp:1617`。
pub unsafe fn lua_concat(l: *mut LuaState, n: i32) {
  unsafe {
    api_check!(l, n >= 0);
    api_checknelems!(l, n);

    if n >= 2 {
      lua_c_check_gc!(l);
      lua_c_threadbarrier_lapi(l);
      lua_v_concat(l, n, ((*l).top.offset_from((*l).base) as i32) - 1);
      (*l).top = (*l).top.sub((n - 1) as usize);
    } else if n == 0 {
      lua_c_threadbarrier_lapi(l);
      // cpp `ensure_stack(L, 1)` 只在 n == 0 分支；n >= 2 由 luaV_concat 自行管栈
      ensure_stack(l, 1);
      setsvalue!(l, (*l).top, lua_s_newlstr(l, &[]));
      api_incr_top!(l);
    }
  }
}
