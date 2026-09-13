use alloc::{ffi::CString, string::String};
use core::ffi::c_void;
use std::panic::catch_unwind;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    install_lua_exception_panic_hook::install_lua_exception_panic_hook,
    lua_g_pusherror::lua_g_pusherror,
  },
  records::lua_exception::lua_exception,
  type_aliases::{lua_state::lua_State, pfunc::Pfunc},
};

#[unsafe(export_name = "ulua_lua_d_rawrunprotected_mut")]
pub(crate) unsafe fn lua_d_rawrunprotected_mut(
  l: *mut lua_State,
  f: Pfunc,
  ud: *mut c_void,
) -> i32 {
  unsafe {
    let mut status: i32 = 0;

    // Silence the default panic-hook noise for the VM's longjmp-emulation
    // unwinds (a caught `lua_exception` is a normal Lua error, not a crash).
    install_lua_exception_panic_hook();

    // In Rust, we use std::panic::catch_unwind to simulate the C++ try/catch boundary.
    // Note: This requires the 'std' library.
    let result = catch_unwind(move || {
      if let Some(f_fn) = f {
        f_fn(l, ud);
      }
    });

    if let Err(payload) = result {
      // Check if the panic payload is a lua_exception.
      if let Some(e) = payload.downcast_ref::<lua_exception>() {
        LUAU_ASSERT!(e.get_thread() == l);
        status = e.get_status();
      } else {
        // Fallback for general panics (equivalent to catch std::exception)
        status = LuaStatus::ErrRun as i32;

        // Best-effort error message: if it's a string-like panic, we could push it,
        // but for parity with the provided skeleton and C++ catch(std::exception),
        // we attempt to push a generic or extracted message.
        let msg = if let Some(s) = payload.downcast_ref::<&str>() {
          *s
        } else if let Some(s) = payload.downcast_ref::<String>() {
          s.as_str()
        } else {
          "unknown Lua error"
        };

        // We need a null-terminated string for luaG_pusherror.
        // Since we are in a panic handler and need to return a status, we use a temporary allocation.
        // Note: std::ffi::CString is used here as this function is already using std::panic.
        let temp_msg = CString::new(msg)
          .unwrap_or_else(|_| CString::new("error message contains null").unwrap());

        lua_g_pusherror(l, temp_msg.as_ptr());
      }
    }

    status
  }
}
