use core::ptr::addr_of;

use crate::{
  enums::lua_status::LuaStatus,
  functions::lua_call::lua_call,
  macros::{
    api_check::api_check, c_call_yield::C_CALL_YIELD, iscfunction::iscfunction,
    isyielded::isyielded,
  },
  records::{closure::CClosure, lua_state::LuaState},
};

/// # Safety
/// `l` 须为存活 `LuaState` 且当前帧 `(*l).ci.func` 为带 `cont` 回调的 C 闭包（`api_check` 校验
/// `iscfunction` 与 `(*c).cont.is_some()`）；`nargs`/`nresults` 须满足 `lua_call` 的栈元素与结果槽约束；
/// 调用可 yield 或抛错，须在受保护帧内调用。cpp `lapi.cpp:1237`（`lua_callyieldable`）。
pub unsafe fn lua_callyieldable(l: *mut LuaState, nargs: i32, nresults: i32) -> i32 {
  unsafe {
    let ci_func = (*(*l).ci).func;
    api_check!(l, iscfunction!(ci_func));
    let cl = (*ci_func).as_closure_ptr();
    let c = addr_of!((*cl).inner.c).cast::<CClosure>();
    api_check!(l, (*c).cont.is_some());

    lua_call(l, nargs, nresults);

    if isyielded(&*l) {
      return C_CALL_YIELD;
    }

    // cont 非空由入口 api_check! 与 pushcclosurek 协议共同保证（cpp: api_check(L, c->cont) 后直接调用）
    let cont = (*c)
      .cont
      .expect("可 yield C 调用闭包的 cont 由 lua_pushcclosurek 协议保证非空");
    cont(l, LuaStatus::Ok as i32)
  }
}
