//! Source: `VM/src/ldebug.cpp:249-254` (hand-ported)

use core::ffi::c_char;

use crate::{
  functions::{cstr_cow, lua_t_objtypename::lua_t_objtypename},
  macros::lua_g_runerror::lua_g_runerror,
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`o` 须指向存活 `TValue`（`lua_t_objtypename` 读其类型），`what` 须为
/// 以 NUL 结尾的合法 C 串（`cstr_cow` 读取）；本函数经 `lua_g_runerror` 格式化消息并抛错，返回 `!`（永不正常返回）。
/// cpp VM/src/ldebug.cpp:283
pub unsafe fn lua_g_forerror_l(l: *mut LuaState, o: *const TValue, what: *const c_char) -> ! {
  unsafe {
    let t: *const c_char = lua_t_objtypename(l, o);

    lua_g_runerror!(
      l,
      "invalid 'for' {} (number expected, got {})",
      cstr_cow(what),
      cstr_cow(t)
    )
  }
}
