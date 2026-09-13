use core::ffi::c_char;

use crate::{
  functions::{lua_g_forerror_l::lua_g_forerror_l, lua_v_tonumber::lua_v_tonumber},
  macros::ttisnumber::ttisnumber,
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

#[unsafe(export_name = "ulua_lua_v_prepare_forn")]
pub(crate) unsafe fn lua_v_prepare_forn(
  l: *mut lua_State,
  plimit: StkId,
  pstep: StkId,
  pinit: StkId,
) {
  unsafe {
    if !ttisnumber!(pinit) && lua_v_tonumber(pinit, pinit).is_null() {
      lua_g_forerror_l(l, pinit, c"initial value".as_ptr() as *const c_char);
    }
    if !ttisnumber!(plimit) && lua_v_tonumber(plimit, plimit).is_null() {
      lua_g_forerror_l(l, plimit, c"limit".as_ptr() as *const c_char);
    }
    if !ttisnumber!(pstep) && lua_v_tonumber(pstep, pstep).is_null() {
      lua_g_forerror_l(l, pstep, c"step".as_ptr() as *const c_char);
    }
  }
}
