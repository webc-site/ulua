use core::{ffi::c_int, ptr::null};

use ulua_vm::{
  functions::{
    lua_l_callyieldable::lua_l_callyieldable, lua_l_checkboolean::lua_l_checkboolean,
    lua_pushcclosurek::lua_pushcclosurek, lua_pushinteger::lua_pushinteger,
    lua_pushnumber::lua_pushnumber, lua_settop::lua_settop,
  },
  records::lua_state::lua_State,
  type_aliases::{lua_c_function::LuaCfunction, lua_continuation::LuaContinuation},
};

use crate::common::functions::{
  nested_multiple_yield_helper::nested_multiple_yield_helper,
  nested_multiple_yield_helper_continuation::nested_multiple_yield_helper_continuation,
  nested_multiple_yield_helper_non_yielding::nested_multiple_yield_helper_non_yielding,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn multiple_yields_with_nested_call(l: *mut lua_State) -> c_int {
  unsafe {
    lua_settop(l, 2);
    let nested_should_yield = lua_l_checkboolean(l, 2) != 0;

    lua_pushinteger(l, 0);
    lua_pushnumber(l, 5.0);

    if nested_should_yield {
      let f: LuaCfunction = Some(nested_multiple_yield_helper);
      let cont: LuaContinuation = Some(nested_multiple_yield_helper_continuation);
      lua_pushcclosurek(l, f, null(), 1, cont);
    } else {
      let f: LuaCfunction = Some(nested_multiple_yield_helper_non_yielding);
      lua_pushcclosurek(l, f, null(), 1, None);
    }

    lua_l_callyieldable(l, 0, 1)
  }
}
