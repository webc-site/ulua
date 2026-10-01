use crate::{
  functions::index_2_addr::index_2_addr,
  macros::{api_check::api_check, lua_utag_limit::LUA_UTAG_LIMIT},
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 LuaState；`idx` 经 `index_2_addr` 解析为指向 full userdata 的栈槽（`api_check` 断言 `is_userdata`，release 由调用方保证），
/// 其 `(*o).value.gc` 所指 Udata 存活可写；`tag` 须 `< LUA_UTAG_LIMIT`（`api_check` 断言），末尾直接写 `(*u).tag`。不抛错/不分配。
/// cpp/VM/src/lapi.cpp:1917 lua_setuserdatatag。
pub unsafe fn lua_setuserdatatag(l: *mut LuaState, idx: i32, tag: i32) {
  unsafe {
    api_check!(l, (tag as u32) < LUA_UTAG_LIMIT as u32);
    let o: StkId = index_2_addr(l, idx);
    api_check!(l, (*o).is_userdata());
    if let Some(u) = (*(*o).value.gc).as_udata_mut() {
      u.tag = tag as u8;
    }
  }
}
