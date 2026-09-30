use crate::{
  functions::{
    ensure_stack::ensure_stack, index_2_addr::index_2_addr, lapi_barrier::lua_c_threadbarrier_lapi,
    lua_h_clone::lua_h_clone,
  },
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, lua_c_check_gc::lua_c_check_gc,
    sethvalue::sethvalue,
  },
  records::{lua_state::LuaState, lua_table::LuaTable},
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 `LuaState`：`idx` 为合法（伪）索引且 `index_2_addr` 所得槽须为 table（`api_check!` is_table），
/// `lua_c_check_gc!`/`lua_h_clone` 可分配并触发 GC，`ensure_stack(l,1)` 后 `sethvalue!` 直写 `(*l).top` 再 `api_incr_top`
/// 抬栈（故 top 前须留 ≥1 槽）；跨线程调用前经 `lua_c_threadbarrier` 同步。
/// cpp VM/src/lapi.cpp:2149
pub unsafe fn lua_clonetable(l: *mut LuaState, idx: i32) {
  unsafe {
    lua_c_check_gc!(l);
    lua_c_threadbarrier_lapi(l);
    // cpp `ensure_stack(L, 1)`：随后 sethvalue 直接写 L->top
    ensure_stack(l, 1);

    let t: StkId = index_2_addr(l, idx);

    api_check!(l, (*t).is_table());

    let tt: *mut LuaTable = lua_h_clone(l, (*t).as_table_ptr());

    sethvalue!(l, (*l).top, tt);
    api_incr_top!(l);
  }
}
