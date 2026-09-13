use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_checktype::lua_l_checktype, lua_l_optinteger::lua_l_optinteger, lua_objlen::lua_objlen,
    lua_pushnil::lua_pushnil, lua_rawgeti::lua_rawgeti, lua_rawseti::lua_rawseti,
    moveelements::moveelements,
  },
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_tremove")]
pub(crate) unsafe extern "C-unwind" fn tremove(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as c_int);
    let n = lua_objlen(l, 1);
    let pos = lua_l_optinteger(l, 2, n);

    if !(1 <= pos && pos <= n) {
      return 0;
    }

    lua_rawgeti(l, 1, pos);

    moveelements(l, 1, 1, pos + 1, n, pos, false);

    lua_pushnil(l);
    lua_rawseti(l, 1, n);

    1
  }
}
