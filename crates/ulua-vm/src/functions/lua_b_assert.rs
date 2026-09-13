use core::ffi::{CStr, c_int};

use crate::{
  functions::{
    lua_gettop::lua_gettop, lua_l_checkany::lua_l_checkany, lua_l_error_l::lua_l_error_l,
    lua_l_optlstring::lua_l_optlstring, lua_toboolean::lua_toboolean,
  },
  type_aliases::lua_state::lua_State,
};

#[unsafe(export_name = "ulua_lua_b_assert")]
pub(crate) unsafe extern "C-unwind" fn lua_b_assert(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checkany(l, 1);
    if lua_toboolean(l, 1) == 0 {
      let mut len = 0;
      let msg = lua_l_optlstring(l, 2, c"assertion failed!".as_ptr(), &mut len);
      let msg = CStr::from_ptr(msg).to_string_lossy();
      lua_l_error_l(l, c"%s".as_ptr(), format_args!("{}", msg));
    }
    lua_gettop(l)
  }
}
