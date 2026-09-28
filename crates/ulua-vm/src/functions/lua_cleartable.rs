use crate::{
  functions::{
    index_2_addr::index_2_addr, lua_g_readonlyerror::check_writable, lua_h_clear::lua_h_clear,
  },
  macros::api_check::api_check,
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于可抛错的受保护帧；`idx` 须经 `index_2_addr` 解析为指向存活 table 的栈槽
/// （`api_check` 断言 `is_table`，release 不校验故须调用方保证），只读表会经 `lua_g_readonlyerror` 抛错，
/// `lua_h_clear` 清空数组与哈希区。cpp/VM/src/lapi.cpp:2139 lua_cleartable。
pub unsafe fn lua_cleartable(l: *mut LuaState, idx: i32) {
  unsafe {
    let t: StkId = index_2_addr(l, idx);
    api_check!(l, (*t).is_table());
    let tt = (*t).as_table_ptr();
    check_writable(l, tt);
    lua_h_clear(tt);
  }
}
