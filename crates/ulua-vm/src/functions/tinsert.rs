use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_gettop::lua_gettop, lua_l_checkinteger::lua_l_checkinteger,
    lua_l_checktype::lua_l_checktype, lua_l_error_l::lua_l_error_l, lua_objlen::lua_objlen,
    lua_rawseti::lua_rawseti, moveelements::moveelements,
  },
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_tinsert")]
pub(crate) unsafe extern "C-unwind" fn tinsert(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as c_int);
    let n = lua_objlen(l, 1);
    let top = lua_gettop(l);
    let pos: i32;

    match top {
      2 => {
        // called with only 2 arguments
        pos = n + 1; // insert new element at the end
      }
      3 => {
        // 2nd argument is the position
        pos = lua_l_checkinteger(l, 2);

        // move up elements if necessary
        if 1 <= pos && pos <= n {
          moveelements(l, 1, 1, pos, n, pos + 1, false);
        }
      }
      _ => {
        lua_l_error_l(
          l,
          c"wrong number of arguments to 'insert'".as_ptr(),
          core::format_args!("wrong number of arguments to 'insert'"),
        );
        return 0;
      }
    }

    lua_rawseti(l, 1, pos); // t[pos] = v
    0
  }
}
