use core::ffi::c_int;

use crate::{
  functions::{
    lua_a_pushclass::luaA_pushclass, lua_a_toobject::luaA_toobject, lua_l_checkany::lua_l_checkany,
    lua_pushnil::lua_pushnil,
  },
  macros::{lua_isobject::lua_isobject, objectvalue::objectvalue},
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

#[unsafe(export_name = "ulua_class_classof")]
pub(crate) unsafe extern "C-unwind" fn class_classof(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checkany(l, 1);

    if !lua_isobject!(l, 1) {
      lua_pushnil(l);
      return 1;
    }

    let inst: *const TValue = luaA_toobject(l, 1);
    let ci = objectvalue!(inst);
    luaA_pushclass(l, ci.lclass);
    1
  }
}
