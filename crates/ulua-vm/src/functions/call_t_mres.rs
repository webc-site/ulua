use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::lua_d_call::lua_d_call,
  macros::{
    lua_d_checkstack::luaD_checkstack, restorestack::restorestack, savestack::savestack,
    setobj_2_s::setobj2s,
  },
  type_aliases::{lua_state::LuaState, stk_id::StkId, t_value::TValue},
};

pub(crate) unsafe fn call_t_mres(
  l: *mut LuaState,
  mut res: StkId,
  f: *const TValue,
  p1: *const TValue,
  p2: *const TValue,
) -> StkId {
  unsafe {
    let result = savestack!(l, res);

    LUAU_ASSERT!((*l).top.offset(3) < (*l).stack.add((*l).stacksize as usize));

    setobj2s!(l, (*l).top, f);
    setobj2s!(l, (*l).top.add(1), p1);
    setobj2s!(l, (*l).top.add(2), p2);

    luaD_checkstack!(l, 3);
    (*l).top = (*l).top.add(3);

    lua_d_call(l, (*l).top.offset(-3), 1);

    res = restorestack!(l, result);
    (*l).top = (*l).top.offset(-1);
    setobj2s!(l, res, (*l).top);

    res
  }
}
