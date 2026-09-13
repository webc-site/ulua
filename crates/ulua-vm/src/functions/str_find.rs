use core::ffi::c_int;

use crate::{functions::str_find_aux::str_find_aux, type_aliases::lua_state::lua_State};

#[unsafe(export_name = "ulua_str_find")]
pub(crate) unsafe extern "C-unwind" fn str_find(l: *mut lua_State) -> c_int {
  unsafe { str_find_aux(l, 1) }
}
