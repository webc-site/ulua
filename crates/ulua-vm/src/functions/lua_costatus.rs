use core::ffi::c_int;

use crate::{
  enums::{lua_co_status::LuaCoStatus, lua_status::LuaStatus},
  macros::api_check::api_check,
  records::lua_state::lua_State,
};

/// # Safety
///
/// `l` and `co` must be valid pointers to live `lua_State` instances belonging to the same global state.
pub unsafe fn lua_costatus(l: *mut lua_State, co: *mut lua_State) -> c_int {
  unsafe {
    api_check!(l, !co.is_null());
    api_check!(l, (*l).global == (*co).global);

    if co == l {
      return LuaCoStatus::CoRun as c_int;
    }
    if (*co).status as i32 == LuaStatus::Yield as i32 {
      return LuaCoStatus::CoSus as c_int;
    }
    if (*co).status as i32 == LuaStatus::Break as i32 {
      return LuaCoStatus::CoNor as c_int;
    }
    if (*co).status != 0 {
      return LuaCoStatus::CoErr as c_int;
    }
    if (*co).ci != (*co).base_ci {
      return LuaCoStatus::CoNor as c_int;
    }
    if (*co).top == (*co).base {
      return LuaCoStatus::CoFin as c_int;
    }
    LuaCoStatus::CoSus as c_int
  }
}
