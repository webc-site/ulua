use core::ffi::c_void;

use crate::records::lua_state::LuaState;
pub type LuaUserdataDirectNamecall = Option<
  unsafe extern "C-unwind" fn(
    l: *mut LuaState,
    data: *mut c_void,
    atom: i32,
    cachedslot: *mut u16,
    utag: i32,
  ) -> i32,
>;
