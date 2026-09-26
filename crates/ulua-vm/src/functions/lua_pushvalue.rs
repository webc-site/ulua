//! Source: `VM/src/lapi.cpp:310-316` (hand-ported)

use crate::{
  functions::{
    ensure_stack::ensure_stack, index_2_addr::index_2_addr, lapi_barrier::lua_c_threadbarrier_lapi,
  },
  macros::{api_incr_top::api_incr_top, setobj_2_s::setobj_2_s},
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于可 GC/可分配（thread barrier）的受保护帧，`ensure_stack(l, 1)` 保证 `(*l).top` 之后留 1 空槽；
/// `idx` 经 `index2addr` 解析为栈内存活 StkId（`setobj2s` 从该槽拷入 `(*l).top` 并 `api_incr_top`）。cpp/VM/src/lapi.cpp:330 lua_pushvalue。
pub unsafe fn lua_pushvalue(l: *mut LuaState, idx: i32) {
  unsafe {
    lua_c_threadbarrier_lapi(l);
    ensure_stack(l, 1);
    let o: StkId = index_2_addr(l, idx);
    setobj_2_s!(l, (*l).top, o);
    api_incr_top!(l);
  }
}
