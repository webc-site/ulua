//! Source: `VM/src/ldebug.cpp:306-311` (hand-ported)

use core::ffi::c_char;

use crate::{
  functions::{cstr_cow, lua_t_objtypename::lua_t_objtypename},
  macros::{getstr::getstr, lua_g_runerror::lua_g_runerror},
  records::{lua_state::LuaState, t_string::tstring},
  type_aliases::t_value::TValue,
};

/// # Safety
/// `l` 须为存活 `LuaState`；`p1` 须指向缺失方法的调用目标 TValue，`p2` 须指向存活 `tstring`
/// （方法名，`getstr` 读其 NUL 结尾 payload），`luaG_runerror` 抛错且永不返回，须受保护帧。cpp `ldebug.cpp:339`。
pub unsafe fn lua_g_methoderror(l: *mut LuaState, p1: *const TValue, p2: *const tstring) -> ! {
  unsafe {
    let t1: *const c_char = lua_t_objtypename(l, p1);

    lua_g_runerror!(
      l,
      "attempt to call missing method '{}' of {}",
      cstr_cow(getstr(p2)),
      cstr_cow(t1)
    )
  }
}
