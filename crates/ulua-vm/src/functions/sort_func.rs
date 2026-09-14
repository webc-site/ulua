use core::ffi::c_int;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::lua_d_call::lua_d_call,
  macros::{l_isfalse::l_isfalse, setobj_2_s::setobj_2_s},
  records::lua_state::lua_State,
  type_aliases::t_value::TValue,
};

pub(crate) unsafe extern "C-unwind" fn sort_func(
  l_state: *mut lua_State,
  l: *const TValue,
  r: *const TValue,
) -> c_int {
  unsafe {
    LUAU_ASSERT!((*l_state).top == (*l_state).base.offset(2)); // table, function

    let top = (*l_state).top;
    let base = (*l_state).base;

    setobj_2_s!(l_state, top, base.offset(1));
    setobj_2_s!(l_state, top.offset(1), l);
    setobj_2_s!(l_state, top.offset(2), r);

    (*l_state).top = top.offset(3); // safe because of LUA_MINSTACK guarantee
    lua_d_call(l_state, top, 1);
    (*l_state).top = (*l_state).top.offset(-1); // maintain stack depth

    (!l_isfalse!((*l_state).top)) as c_int
  }
}
