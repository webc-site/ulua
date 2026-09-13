use core::ffi::c_int;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_checkstack::lua_checkstack, lua_l_error_l::lua_l_error_l,
    lua_rawcheckstack::lua_rawcheckstack, lua_xmove::lua_xmove,
  },
  macros::{cast_int::cast_int, co_status_error::CO_STATUS_ERROR},
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_auxresumecont")]
pub(crate) unsafe fn auxresumecont(l: *mut lua_State, co: *mut lua_State) -> c_int {
  unsafe {
    if (*co).status == LuaStatus::Ok as u8 || (*co).status == LuaStatus::Yield as u8 {
      let nres = cast_int!((*co).top.offset_from((*co).base));
      if lua_checkstack(l, nres + 1) == 0 {
        lua_l_error_l(
          l,
          c"too many results to resume".as_ptr(),
          format_args!("too many results to resume"),
        );
      }
      lua_xmove(co, l, nres);
      nres
    } else {
      lua_rawcheckstack(l, 2);
      lua_xmove(co, l, 1);
      CO_STATUS_ERROR
    }
  }
}
