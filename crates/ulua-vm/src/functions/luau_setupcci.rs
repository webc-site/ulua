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
  // SAFETY: 契约保证 `l` 的 CallInfo 数组尚有空槽（上界由调用方检查）且 fun..top 落在栈数组界内，ci.top 预留 LUA_MINSTACK
  unsafe {
    let ci = incr_ci!(l);

    (*ci).func = fun;
    (*ci).base = fun.add(1);
    // 保留（r12-w9b 逐点定性，w6d 口径钉死合法）：本行为 CallInfo 帧建立面的裸字段
    // 落库（帧 ABI 本体，slot.rs 红线），且算术操作数 `top + LUA_MINSTACK` 求值于
    // 下方 lua_d_checkstackfornewci 扩容之前——正向界彼时尚未被扩容先行覆盖，不满足
    // `top_slot` 读数原语的分配界内契约；形制同已判保留的 lua_v_call_tm 建立点与
    // 豁免面 luau_callhook/luau_precall 同款建立点，不强收。
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
