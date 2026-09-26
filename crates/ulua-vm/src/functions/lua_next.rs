//! Source: `VM/src/lapi.cpp:1350-1363` (hand-ported)

use crate::{
  functions::{
    ensure_stack::ensure_stack, index_2_addr::index_2_addr, lapi_barrier::lua_c_threadbarrier_lapi,
    lua_h_next::lua_h_next,
  },
  macros::{api_check::api_check, api_checknelems::api_checknelems, api_incr_top::api_incr_top},
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 `LuaState`：`api_checknelems!(l,1)` 要求 `(*l).top` 前已压 1 个 key（读 `top-1`），`idx` 为合法索引且
/// `index_2_addr` 所得槽为 table（`api_check!` is_table，`hvalue` 后存活）；`ensure_stack(l,1)` 保证有 key 时 `lua_h_next`
/// 可写 `top` 并 `api_incr_top`、无 key 时回退 `top-1`。跨线程经 threadbarrier 同步；`lua_h_next` 可能触发 GC。
/// cpp VM/src/lapi.cpp:1553
pub unsafe fn lua_next(l: *mut LuaState, idx: i32) -> i32 {
  unsafe {
    api_checknelems!(l, 1);
    lua_c_threadbarrier_lapi(l);
    ensure_stack(l, 1);
    let t: StkId = index_2_addr(l, idx);
    api_check!(l, (*t).is_table());

    let more = lua_h_next(l, (*t).as_table_ptr(), (*l).top.sub(1));
    if more != 0 {
      api_incr_top!(l);
    } else {
      (*l).top = (*l).top.sub(1);
    }
    more
  }
}
