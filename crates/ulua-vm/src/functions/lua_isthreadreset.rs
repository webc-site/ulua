use core::ffi::c_int;

use crate::{enums::lua_status::LuaStatus, type_aliases::lua_state::lua_State};

#[unsafe(export_name = "ulua_lua_isthreadreset")]
pub(crate) unsafe fn lua_isthreadreset(l: *mut lua_State) -> c_int {
  unsafe {
    ((*l).ci == (*l).base_ci && (*l).base == (*l).top && (*l).status == LuaStatus::Ok as u8)
      as c_int
  }
}
