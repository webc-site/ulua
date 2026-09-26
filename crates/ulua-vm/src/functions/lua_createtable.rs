use crate::{
  functions::{
    ensure_stack::ensure_stack, lapi_barrier::lua_c_threadbarrier_lapi, lua_h_new::lua_h_new,
  },
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, lua_c_check_gc::lua_c_check_gc,
    sethvalue::sethvalue,
  },
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState`；`narray`/`nrec` 均须 `>=0`（`api_check`），`lua_h_new` 按此分配并可能
/// 触发 GC（`lua_c_check_gc`）、`ensure_stack(l,1)` 可能重分配栈、`sethvalue` 写 `(*l).top` 并可挂屏障，
/// 须在受保护帧内调用。cpp `lapi.cpp:897`。
pub unsafe fn lua_createtable(l: *mut LuaState, narray: i32, nrec: i32) {
  unsafe {
    api_check!(l, narray >= 0 && nrec >= 0);
    lua_c_check_gc!(l);
    lua_c_threadbarrier_lapi(l);
    ensure_stack(l, 1);
    sethvalue!(l, (*l).top, lua_h_new(l, narray, nrec));
    api_incr_top!(l);
  }
}
