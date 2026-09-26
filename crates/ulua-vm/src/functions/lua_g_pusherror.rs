use core::ffi::c_char;

use crate::{
  functions::{lua_rawcheckstack::lua_rawcheckstack, pusherror::pusherror},
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_g_pusherror(l: *mut LuaState, error: *const c_char) {
  unsafe {
    lua_rawcheckstack(l, 1);

    let empty = c"";
    let error = if error.is_null() {
      empty.as_ptr()
    } else {
      error
    };
    pusherror(l, error);
  }
}
