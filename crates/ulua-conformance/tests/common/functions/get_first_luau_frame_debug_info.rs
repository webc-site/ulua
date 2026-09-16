use core::{ffi::CStr, mem::zeroed};

use ulua_vm::{
  functions::lua_getinfo::lua_getinfo,
  records::{lua_debug::LuaDebug, lua_state::lua_State},
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn get_first_luau_frame_debug_info(l: *mut lua_State) -> Option<LuaDebug> {
  unsafe {
    let mut level = 0;

    loop {
      let mut ar: LuaDebug = zeroed();
      if lua_getinfo(l, level, c"sl".as_ptr(), &mut ar) == 0 {
        return None;
      }

      if !ar.what.is_null() && CStr::from_ptr(ar.what).to_bytes() == b"Lua" {
        return Some(ar);
      }

      level += 1;
    }
  }
}
