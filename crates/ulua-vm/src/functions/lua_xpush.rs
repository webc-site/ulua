//! Source: `VM/src/lapi.cpp:206-212` (hand-ported)

use crate::{
  functions::{
    ensure_stack::ensure_stack_impl, index_2_addr::index_2_addr,
    lapi_barrier::lua_c_threadbarrier_lapi,
  },
  macros::{api_check::api_check, api_incr_top::api_incr_top, setobj_2_s::setobj_2_s},
  records::lua_state::LuaState,
};

/// # Safety
/// `from` 与 `to` 须为同属一个 `global`（`api_check (*from).global==(*to).global`）的存活 `LuaState`；
/// `idx` 须为 `from` 的合法（伪）索引、解析出指向存活 TValue 的指针；`ensure_stack_impl(to,from,1)` 会扩
/// `to` 栈、`setobj2s` 写入 `(*to).top` 并可能触发 GC/屏障，须在受保护帧内调用。cpp `lapi.cpp:224`。
pub unsafe fn lua_xpush(from: *mut LuaState, to: *mut LuaState, idx: i32) {
  unsafe {
    api_check!(from, (*from).global == (*to).global);
    lua_c_threadbarrier_lapi(to);
    // cpp `ensure_stack_impl(to, from, 1)`
    ensure_stack_impl(to, from, 1);
    let o = index_2_addr(from, idx);
    setobj_2_s!(to, (*to).top, o);
    api_incr_top!(to);
  }
}
