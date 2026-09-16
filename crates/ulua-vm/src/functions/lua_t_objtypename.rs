use core::ffi::c_char;

use crate::{
  functions::lua_t_objtypenamestr::lua_t_objtypenamestr,
  macros::getstr::getstr,
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

pub(crate) unsafe fn lua_t_objtypename(l: *mut lua_State, o: *const TValue) -> *const c_char {
  unsafe { getstr(lua_t_objtypenamestr(l, o)) }
}
