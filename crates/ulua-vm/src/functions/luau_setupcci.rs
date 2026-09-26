//! Source: `VM/src/lvmexecute.cpp:206`
//!
//! Push and initialize a fresh `CallInfo` for a C continuation call: point it at
//! `fun`, give it a `LUA_MINSTACK` window, clear the saved pc/flags, and ensure
//! the stack has room.

use core::ptr::null;

use ulua_common::LUAU_ASSERT;

use crate::{
  macros::{
    incr_ci::incr_ci, lua_d_checkstackfornewci::lua_d_checkstackfornewci,
    lua_minstack::LUA_MINSTACK,
  },
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 调用方须保证：CallInfo 数组至少余一空槽（incr_ci! 不做越界检查，溢出即踩内存）；`fun` 槽
/// 当前为函数值（末尾断言，release 下不检查）；`(*l).top..top+LUA_MINSTACK` 落在栈数组内或可被
/// lua_d_checkstackfornewci 扩容容纳；`nresults` 为续体（continuation）预留的结果槽数。
/// cpp lvmexecute.cpp:225 `luau_setupcci`
pub(crate) unsafe fn luau_setupcci(l: *mut LuaState, nresults: i32, fun: StkId) {
  // Safety: 契约保证 `l` 的 CallInfo 数组尚有空槽（上界由调用方检查）且 fun..top 落在栈数组界内，ci.top 预留 LUA_MINSTACK
  unsafe {
    let ci = incr_ci!(l);

    (*ci).func = fun;
    (*ci).base = fun.add(1);
    (*ci).top = (*l).top.add(LUA_MINSTACK as usize);
    (*ci).savedpc = null();
    (*ci).flags = 0;
    (*ci).nresults = nresults;

    (*l).base = fun.add(1);

    lua_d_checkstackfornewci(l, LUA_MINSTACK);

    LUAU_ASSERT!((*ci).top <= (*l).stack_last);
    LUAU_ASSERT!((*(*ci).func).is_function());
  }
}
