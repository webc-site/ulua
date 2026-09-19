use core::ffi::{c_int, c_void};

use ulua_vm::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_checkunsigned::lua_l_checkunsigned, lua_topointer::lua_topointer, lua_type::lua_type,
  },
  macros::lua_pushlightuserdata::lua_pushlightuserdata,
  records::lua_state::lua_State,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_tables_make_lud(l: *mut lua_State) -> c_int {
  unsafe {
    if lua_type(l, 1) == LuaType::Number as c_int {
      let v = lua_l_checkunsigned(l, 1);
      lua_pushlightuserdata(l, v as usize as *mut c_void);
    } else {
      let p = lua_topointer(l, 1);
      assert!(!p.is_null());
      lua_pushlightuserdata(l, p as *mut c_void);
    }

    1
  }
}
