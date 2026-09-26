use crate::{
  functions::{
    getluaproto::get_lua_proto, lapi_barrier::lua_c_threadbarrier_lapi,
    lua_a_pushvalue::lua_a_pushvalue,
  },
  macros::lua_callinfo_native::LUA_CALLINFO_NATIVE,
  records::{call_info::CallInfo, lua_state::LuaState, proto::Proto},
};

/// # Safety
/// `l` 须为存活 `LuaState`：`level` 须满足 `0≤level<(*l).ci-(*l).base_ci`（越界提前返回 0），据此偏移出的 `ci`
/// 须为存活帧；非 NATIVE 帧时经 `get_lua_proto` 取 `Proto`（Lua 帧非空），读 `numparams/is_vararg`；
/// 取参时 `(*ci).base+(n-1)` 或 `(*ci).func+n` 须落在该帧 `[func,top)` 实参区间内方可 `lua_a_pushvalue` 压栈（需 `(*l).top` 后空槽，可 GC）。
/// cpp VM/src/ldebug.cpp:44
pub unsafe fn lua_getargument(l: *mut LuaState, level: i32, n: i32) -> i32 {
  unsafe {
    if (level as u32) >= ((*l).ci.offset_from((*l).base_ci) as u32) {
      return 0;
    }

    let ci: *mut CallInfo = (*l).ci.offset(-(level as isize));

    // changing tables in native functions externally may invalidate safety contracts wrt table state (metatable/size/readonly)
    if ((*ci).flags & LUA_CALLINFO_NATIVE as u32) != 0 {
      return 0;
    }

    let fp: *mut Proto = get_lua_proto(ci);
    let mut res: i32 = 0;

    if !fp.is_null() && n > 0 {
      if (n as u32) <= (*fp).numparams as u32 {
        lua_c_threadbarrier_lapi(l);
        lua_a_pushvalue(l, (*ci).base.offset((n - 1) as isize));
        res = 1;
      } else if (*fp).is_vararg != 0 && (n as isize) < (*ci).base.offset_from((*ci).func) {
        lua_c_threadbarrier_lapi(l);
        lua_a_pushvalue(l, (*ci).func.offset(n as isize));
        res = 1;
      }
    }

    res
  }
}
