use core::ffi::{c_int, c_void};

use crate::type_aliases::lua_state::lua_State;
pub type LuaUserdataDirectNamecall = Option<
  unsafe extern "C-unwind" fn(
    l: *mut lua_State,
    data: *mut c_void,
    atom: c_int,
    cachedslot: *mut u16,
    utag: c_int,
  ) -> c_int,
>;
