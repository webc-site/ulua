use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_call::lua_call, lua_l_checktype::lua_l_checktype, lua_objlen::lua_objlen,
    lua_pushinteger::lua_pushinteger, lua_pushvalue::lua_pushvalue, lua_rawgeti::lua_rawgeti,
  },
  macros::{lua_isnil::lua_isnil, lua_pop::lua_pop},
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_foreachi")]
pub(crate) unsafe extern "C-unwind" fn foreachi(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as c_int);
    lua_l_checktype(l, 2, LuaType::Function as c_int);

    let mut i: i32 = 1;
    let n = lua_objlen(l, 1);

    while i <= n {
      lua_pushvalue(l, 2); // function
      lua_pushinteger(l, i); // 1st argument
      lua_rawgeti(l, 1, i); // 2nd argument
      lua_call(l, 2, 1);

      if !lua_isnil!(l, -1) {
        return 1;
      }
      lua_pop(l, 1); // remove nil result

      i += 1;
    }

    0
  }
}
