use crate::{
  functions::{
    index_2_addr::index_2_addr, lua_g_readonlyerror::check_writable, lua_h_setnum::lua_h_setnum,
  },
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, lua_c_barriert::luaC_barriert,
    setobj_2_t::setobj2t,
  },
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于可 GC/可抛错的受保护帧，栈顶已压入 1 个 value（`api_checknelems!(l, 1)`，由调用方保证）；
/// `idx` 经 `index_2_addr` 解析为指向存活非只读 table 的栈槽（`is_table` 断言，`readonly` 时 `lua_g_readonlyerror` 抛错）；
/// `luaH_setnum`/`setobj2t`/`luaC_barriert` 写表并可能触发写屏障/重哈希，最后 `top` 回退一格消费 value。cpp/VM/src/lapi.cpp:1043 lua_rawseti。
pub unsafe fn lua_rawseti(l: *mut LuaState, idx: i32, n: i32) {
  unsafe {
    api_checknelems!(l, 1);
    let o: StkId = index_2_addr(l, idx);
    api_check!(l, (*o).is_table());
    check_writable(l, (*o).as_table_ptr());
    // 栈顶 value 槽的三连裸重读收为一次预绑定（setnum/rehash/GC 均不改写 `(*l).top`）
    let value = (*l).top.offset(-1);
    setobj2t!(l, lua_h_setnum(l, (*o).as_table_ptr(), n), value);
    luaC_barriert!(l, (*o).as_table_ptr(), value);
    (*l).top = value;
  }
}
