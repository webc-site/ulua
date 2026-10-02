use core::ffi::c_char;

use crate::{
  functions::{cstr_bytes, lua_rawcheckstack::lua_rawcheckstack, pusherror::pusherror_bytes},
  records::lua_state::LuaState,
};

/// 安全字节切片版错误信息压栈。
///
/// r16-v7 步骤 2：同款接收者前移 `*mut` → `&mut LuaState`——体即 safe 引用形的
/// `lua_rawcheckstack` + 本票步骤 1 转 safe 的 `pusherror_bytes`，整体零 unsafe；
/// 检查栈先于压错的次序逐位不变。
pub(crate) fn lua_g_pusherror_bytes(l: &mut LuaState, error: &[u8]) {
  lua_rawcheckstack(l, 1);
  pusherror_bytes(l, error);
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_g_pusherror(l: *mut LuaState, error: *const c_char) {
  unsafe {
    lua_g_pusherror_bytes(&mut *l, cstr_bytes(error));
  }
}
