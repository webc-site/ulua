use crate::{
  functions::{
    index_2_addr::index_2_addr, lapi_barrier::lua_c_threadbarrier_lapi, lua_h_get::lua_h_get,
  },
  macros::{api_check::api_check, setobj_2_s::setobj_2_s, ttype::ttype},
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 `LuaState`：`idx` 为合法索引且 `index_2_addr` 所得槽为 table（`api_check!` is_table）；调用前
/// `(*l).top-1` 须已压入 key 槽（读该槽、`lua_h_get` 后原地 `setobj_2_s` 覆写为 value，不额外抬栈，故 key 位即为返回 value 位）。
/// 跨线程经 threadbarrier 同步；`lua_h_get` 只读不触发 GC。返回压回值类型。
/// cpp VM/src/lapi.cpp:866
pub unsafe fn lua_rawget(l: *mut LuaState, idx: i32) -> i32 {
  unsafe {
    lua_c_threadbarrier_lapi(l);

    let t: StkId = index_2_addr(l, idx);
    api_check!(l, (*t).is_table());

    let slot = (*l).top.sub(1);
    setobj_2_s!(l, slot, lua_h_get((*t).as_table_ptr(), slot));

    ttype!((*l).top.sub(1)) as i32
  }
}
