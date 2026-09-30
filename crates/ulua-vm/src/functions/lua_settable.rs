use core::ptr::eq;

use crate::{
  functions::{index_2_addr::index_2_addr, lua_v_settable::lua_v_settable},
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, lua_o_nilobject::LUA_O_NILOBJECT,
  },
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于可 GC/可抛错的受保护帧，栈顶已压入 key+value 两项（`api_checknelems!(l, 2)`，release 由调用方保证）；
/// `idx` 经 `index_2_addr` 解析为非 `LUA_O_NILOBJECT` 的栈槽（`api_check`）；`lua_v_settable` 以 top-2 为 key、top-1 为 value 写入，
/// 末尾 `top` 回退两格。cpp/VM/src/lapi.cpp:999 lua_settable。
pub unsafe fn lua_settable(l: *mut LuaState, idx: i32) {
  unsafe {
    api_checknelems!(l, 2);

    let t: StkId = index_2_addr(l, idx);
    api_check!(l, !eq(t, LUA_O_NILOBJECT));
    lua_v_settable(l, t, (*l).top.sub(2), (*l).top.sub(1));
    (*l).top = (*l).top.sub(2);
  }
}
