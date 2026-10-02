use core::ffi::c_char;

use crate::{
  functions::{cstr_bytes, lua_rawcheckstack::lua_rawcheckstack, pusherror::pusherror_bytes},
  records::lua_state::LuaState,
};

/// 安全字节切片版错误信息压栈。
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn lua_g_pusherror_bytes(l: *mut LuaState, error: &[u8]) {
  unsafe {
    lua_rawcheckstack(l, 1);
    pusherror_bytes(l, error);
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_g_pusherror(l: *mut LuaState, error: *const c_char) {
  unsafe {
    lua_g_pusherror_bytes(l, cstr_bytes(error));
  }
}
