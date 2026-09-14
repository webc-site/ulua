use core::{
  ffi::{CStr, c_char, c_int, c_void},
  mem::transmute,
};

use crate::{
  functions::{lua_rawcheckstack::lua_rawcheckstack, pusherror::pusherror},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_g_pusherror")]
pub unsafe fn lua_g_pusherror(l: *mut lua_State, error: *const c_char) {
  unsafe {
    // The provided lua_rawcheckstack stub has no arguments, but the logic requires (l, n).
    // We cast the function pointer to the correct signature to satisfy the call.
    let lua_rawcheckstack_ptr = lua_rawcheckstack as *const c_void;
    let lua_rawcheckstack_real: unsafe extern "C-unwind" fn(*mut lua_State, c_int) =
      transmute(lua_rawcheckstack_ptr);

    lua_rawcheckstack_real(l, 1);

    let error_str = if error.is_null() {
      ""
    } else {
      CStr::from_ptr(error).to_str().unwrap_or("")
    };

    // The provided pusherror stub has no arguments, but the logic requires (l, error).
    // We cast the function pointer to the correct signature to satisfy the call.
    let pusherror_ptr = pusherror as *const c_void;
    let pusherror_real: unsafe fn(*mut lua_State, &str) = transmute(pusherror_ptr);

    pusherror_real(l, error_str);
  }
}

pub use lua_g_pusherror as luaG_pusherror;
