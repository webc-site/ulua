//! Source: `VM/src/ldebug.cpp:242-247` (hand-ported)

use core::ffi::c_char;

use crate::{
  functions::{cstr_cow, lua_t_objtypename::lua_t_objtypename},
  macros::lua_g_runerror::lua_g_runerror,
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// # Safety
/// `l` 须为存活 `LuaState`；`o` 须指向参与运算的存活 TValue（`luaT_objtypename` 读其类型名），
/// `op` 为静态消息词 `&str`；`luaG_runerror` 抛错且永不返回，须受保护帧。cpp `ldebug.cpp:276`。
pub unsafe fn lua_g_typeerror_l(l: *mut LuaState, o: *const TValue, op: &str) -> ! {
  unsafe {
    // 类型名来自 lua 对象内部，仍是 C 字符串；op 为固定消息词，用 &str
    let t: *const c_char = lua_t_objtypename(l, o);

    lua_g_runerror!(l, "attempt to {} a {} value", op, cstr_cow(t))
  }
}
