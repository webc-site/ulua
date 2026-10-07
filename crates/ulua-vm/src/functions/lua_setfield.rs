use core::ptr::eq;

use crate::{
  functions::{index_2_addr::index_2_addr, lua_v_settable::lua_v_settable},
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, lua_o_nilobject::LUA_O_NILOBJECT,
    lua_s_new::lua_s_new, setsvalue::setsvalue,
  },
  records::{lua_state::LuaState, lua_t_value::TValue, slot::Slot},
  type_aliases::stk_id::StkId,
};

/// `lua_setfield` 核心（cpp `lapi.cpp:1010`）。调用序契约（正确性，非内存安全）：
/// `l` 处于可 GC/可抛错的受保护帧，栈顶已压 value 一项（`api_checknelems!(l, 1)`）；
/// `idx` 经 `index_2_addr` 解析为非 `LUA_O_NILOBJECT` 的栈槽（`api_check`）；`k` 为字段名字节切片
/// （intern 可分配），写入后 `top` 回退一格。
pub(crate) fn lua_setfield_bytes(l: &mut LuaState, idx: i32, k: &[u8]) {
  unsafe {
    api_checknelems!(l, 1);

    let t: StkId = index_2_addr(l, idx);
    api_check!(l, !eq(t, LUA_O_NILOBJECT));

    let mut key = TValue::default();
    setsvalue!(l, &mut key, lua_s_new(l, k));
    lua_v_settable(
      l,
      Slot::from_raw(t),
      Slot::from_ref(&key),
      Slot::from_raw(l.top.sub(1)),
    );
    // 消费 value 一格：`rewind_top(1)` 提交原语镜像原 `top = top.sub(1)` 落值
    l.rewind_top(1);
  }
}
