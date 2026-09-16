use core::mem::zeroed;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::lua_v_gettable::lua_v_gettable,
  macros::{
    restorestack::restorestack, savestack::savestack, sethvalue::sethvalue, ttisnil::ttisnil,
  },
  type_aliases::{lua_state::lua_State, lua_table::LuaTable, stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_v_getimport(
  l: *mut lua_State,
  env: *mut LuaTable,
  k: *mut TValue,
  mut res: StkId,
  id: u32,
  propagatenil: bool,
) {
  unsafe {
    let count = id >> 30;
    LUAU_ASSERT!(count > 0);

    let id0 = ((id >> 20) & 1023) as usize;
    let id1 = ((id >> 10) & 1023) as usize;
    let id2 = (id & 1023) as usize;

    // after the first call to luaV_gettable, res may be invalid, and env may (sometimes) be garbage collected
    // we take care to not use env again and to restore res before every consecutive use
    let resp = savestack!(l, res);

    // global lookup for id0
    let mut g: TValue = zeroed();
    sethvalue!(l, &mut g as *mut TValue, env);
    lua_v_gettable(l, &g as *const TValue, k.add(id0), res);

    // table lookup for id1
    if count < 2 {
      return;
    }

    res = restorestack!(l, resp);
    if !propagatenil || !ttisnil!(res) {
      lua_v_gettable(l, res as *const TValue, k.add(id1), res);
    }

    // table lookup for id2
    if count < 3 {
      return;
    }

    res = restorestack!(l, resp);
    if !propagatenil || !ttisnil!(res) {
      lua_v_gettable(l, res as *const TValue, k.add(id2), res);
    }
  }
}
