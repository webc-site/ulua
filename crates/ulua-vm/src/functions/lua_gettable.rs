use core::ptr::eq;

use crate::{
  functions::{
    index_2_addr::index_2_addr, lapi_barrier::lua_c_threadbarrier_lapi,
    lua_v_gettable::lua_v_gettable,
  },
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, lua_o_nilobject::LUA_O_NILOBJECT,
    ttype::ttype,
  },
  records::{lua_state::LuaState, slot::Slot},
  type_aliases::stk_id::StkId,
};

/// `lua_gettable` 核心（cpp/VM/src/lapi.cpp:830）。调用序契约（正确性，非内存安全）：
/// `l` 处于可 GC/可抛错的受保护帧，栈顶已压入 1 个 key（`api_checknelems!(l, 1)`，release 不校验故由调用方保证）；
/// `idx` 经 `index_2_addr` 解析为指向可索引值的栈槽且非 `LUA_O_NILOBJECT`（`api_check`）；`lua_v_gettable` 以 top-1 为 key、原地写回结果。
pub fn lua_gettable(l: &mut LuaState, idx: i32) -> i32 {
  unsafe {
    api_checknelems!(l, 1);
    lua_c_threadbarrier_lapi(l);

    let t: StkId = index_2_addr(l, idx);
    api_check!(l, !eq(t, LUA_O_NILOBJECT));
    lua_v_gettable(
      l,
      Slot::from_raw(t),
      Slot::from_raw(l.top.sub(1)),
      Slot::from_raw(l.top.sub(1)),
    );

    ttype!(l.top.sub(1)) as i32
  }
}
