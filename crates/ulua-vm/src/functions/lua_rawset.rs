use crate::{
  functions::{
    index_2_addr::index_2_addr, lua_g_readonlyerror::check_writable, lua_h_set::lua_h_set,
  },
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, lua_c_barriert::luaC_barriert,
    setobj_2_t::setobj2t,
  },
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 `LuaState`：`api_checknelems!(l,2)` 要求 `(*l).top` 前已压 key、value 两槽（读 `top-2`、`top-1`）；
/// `idx` 为合法索引且 `index_2_addr` 所得槽为 table（is_table），其 `(*hvalue).readonly` 须为 0 否则 `lua_g_readonlyerror` 抛错回退；
/// `lua_h_set` 可能 rehash 返回可写 slot（`setobj2t` 写回、`luaC_barriert` 做写屏障防漏灰），完成后 `(*l).top` 回退 2。可触发 GC。
/// cpp VM/src/lapi.cpp:1031
pub unsafe fn lua_rawset(l: *mut LuaState, idx: i32) {
  unsafe {
    api_checknelems!(l, 2);
    let t: StkId = index_2_addr(l, idx);
    api_check!(l, (*t).is_table());
    check_writable(l, (*t).as_table_ptr());
    let key = (*l).top.offset(-2);
    let value = (*l).top.offset(-1);
    // ⇔ cpp lapi.cpp:1038-1040 逐位同序：:1038 `setobj2t(L, luaH_set(...), top-1)`
    // （取槽→写值，key/value 是栈槽指针，不受表 rehash 影响）、:1039 `luaC_barriert`
    // （屏障在值落槽之后）、:1040 `top -= 2`。屏障与写分步保留，不收敛单函数。
    let slot = lua_h_set(l, (*t).as_table_ptr(), key);
    setobj2t!(l, slot, value);
    luaC_barriert!(l, (*t).as_table_ptr(), value);
    (*l).top = key;
  }
}
