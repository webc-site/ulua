use core::ffi::c_int;

use ulua_vm::{
  functions::lua_l_error_l::lua_l_error_l, macros::lua_use_longjmp::LUA_USE_LONGJMP,
  records::lua_state::lua_State,
};

/// C-unwind ABI：LUA_USE_LONGJMP=0 时跨 C 栈帧 panic 展开是定义行为。
pub(crate) extern "C-unwind" fn cxxthrow(l: *mut lua_State) -> c_int {
  unsafe {
    if LUA_USE_LONGJMP != 0 {
      lua_l_error_l(l, c"oops".as_ptr(), core::format_args!("oops"));
      0
    } else {
      panic!("oops");
    }
  }
}
