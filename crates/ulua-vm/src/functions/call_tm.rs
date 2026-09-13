use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::lua_d_call::lua_d_call,
  macros::{lua_d_checkstack::luaD_checkstack, setobj_2_s::setobj2s},
  type_aliases::{lua_state::LuaState, t_value::TValue},
};

pub(crate) unsafe fn call_tm(
  l: *mut LuaState,
  f: *const TValue,
  p1: *const TValue,
  p2: *const TValue,
  p3: *const TValue,
) {
  unsafe {
    LUAU_ASSERT!((*l).top.offset(4) < (*l).stack.add((*l).stacksize as usize));

    setobj2s!(l, (*l).top, f);
    setobj2s!(l, (*l).top.add(1), p1);
    setobj2s!(l, (*l).top.add(2), p2);
    setobj2s!(l, (*l).top.add(3), p3);

    luaD_checkstack!(l, 4);
    (*l).top = (*l).top.add(4);

    lua_d_call(l, (*l).top.offset(-4), 0);
  }
}
