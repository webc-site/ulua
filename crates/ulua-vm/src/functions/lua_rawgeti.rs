use crate::{
  functions::{
    ensure_stack::ensure_stack, index_2_addr::index_2_addr, lapi_barrier::lua_c_threadbarrier_lapi,
    lua_h_getnum::lua_h_getnum,
  },
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, setobj_2_s::setobj_2_s, ttype::ttype,
  },
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于可 GC 的受保护帧，`ensure_stack(l, 1)` 留 1 结果槽；`idx` 经 `index_2_addr` 解析为指向
/// 存活 table 的栈槽（`api_check` 断言 `is_table`，release 由调用方保证），`hvalue` 转出的 LuaTable 的数组/哈希区自洽；
/// `lua_h_getnum` 返回值经 `setobj2s` 写入 top 并 `api_incr_top`。cpp/VM/src/lapi.cpp:875 lua_rawgeti。
pub unsafe fn lua_rawgeti(l: *mut LuaState, idx: i32, n: i32) -> i32 {
  unsafe {
    lua_c_threadbarrier_lapi(l);
    ensure_stack(l, 1);

    let t: StkId = index_2_addr(l, idx);
    api_check!(l, (*t).is_table());

    setobj_2_s!(l, (*l).top, lua_h_getnum((*t).as_table_ptr(), n));
    api_incr_top!(l);

    ttype!((*l).top.sub(1)) as i32
  }
}
