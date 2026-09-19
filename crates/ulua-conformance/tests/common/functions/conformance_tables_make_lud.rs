use core::ffi::{c_int, c_void};

use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_checkunsigned::lua_l_checkunsigned, lua_topointer::lua_topointer, lua_type::lua_type,
  },
  macros::LUA_PUSHLIGHTUSERDATA::LUA_PUSHLIGHTUSERDATA,
  records::lua_state::lua_State,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_tables_make_lud(l: *mut lua_State) -> c_int {
  unsafe {
    if lua_type(l, 1) == LuaType::Number as c_int {
      let v = lua_l_checkunsigned(l, 1);
      LUA_PUSHLIGHTUSERDATA(l as *mut c_void, v as usize as *mut c_void);
    } else {
      let p = lua_topointer(l, 1);
      assert!(!p.is_null());
      LUA_PUSHLIGHTUSERDATA(l as *mut c_void, p as *mut c_void);
    }

    1
  }
}
