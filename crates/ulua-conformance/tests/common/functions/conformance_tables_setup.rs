use core::ffi::c_int;

use ulua_vm::{
  macros::{lua_pushcfunction::LUA_PUSHCFUNCTION, lua_setglobal::lua_setglobal},
  records::lua_state::lua_State,
  type_aliases::lua_c_function::LuaCfunction,
};

use crate::common::functions::conformance_tables_make_lud::conformance_tables_make_lud;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_tables_setup(l: *mut lua_State) {
  unsafe {
    let make_lud: LuaCfunction =
      Some(conformance_tables_make_lud as unsafe extern "C-unwind" fn(*mut lua_State) -> c_int);
    LUA_PUSHCFUNCTION(l, make_lud, c"makelud".as_ptr());
    lua_setglobal(l, c"makelud".as_ptr());
  }
}
