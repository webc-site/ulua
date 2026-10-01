use core::ptr::eq;

use crate::{
  functions::{
    ensure_stack::ensure_stack, index_2_addr::index_2_addr, lapi_barrier::lua_c_threadbarrier_lapi,
    lua_v_gettable::lua_v_gettable,
  },
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, lua_o_nilobject::LUA_O_NILOBJECT,
    lua_s_new::lua_s_new, setsvalue::setsvalue, ttype::ttype,
  },
  records::{lua_state::LuaState, lua_t_value::TValue, slot::Slot},
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于可分配/GC/可抛错的受保护帧，`(*l).top` 之后经 `ensure_stack(l, 1)` 留 1 空槽（结果写入并 `api_incr_top`）；
/// `idx` 经 `index_2_addr` 解析为指向可索引值（table/带 __index 元表者）的栈槽且非 `LUA_O_NILOBJECT`（`api_check`）；
/// `k` 为字节切片，`lua_s_new` 会 intern（可分配）。
pub(crate) unsafe fn lua_getfield_bytes(l: *mut LuaState, idx: i32, k: &[u8]) -> i32 {
  unsafe {
    lua_c_threadbarrier_lapi(l);
    ensure_stack(l, 1);

    let t: StkId = index_2_addr(l, idx);
    api_check!(l, !eq(t, LUA_O_NILOBJECT));

    let mut key = TValue::default();
    setsvalue!(l, &mut key, lua_s_new(l, k));
    lua_v_gettable(
      l,
      Slot::from_raw(t),
      Slot::from_mut(&mut key),
      Slot::from_raw((*l).top),
    );
    api_incr_top!(l);

    ttype!((*l).top.sub(1)) as i32
  }
}
