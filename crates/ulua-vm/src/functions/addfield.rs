//! Source: `VM/src/ltablib.cpp:230`
//!
//! Helper for `table.concat` — append element `i` of table `t` to the buffer.
//! Fast path reads a string directly from the array part; otherwise it goes
//! through `lua_rawgeti` and rejects non-string/number values.

use core::slice::from_raw_parts;

use ulua_common::functions::c_str::cstr_cow;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_l_addlstring::lua_l_addlstring, lua_l_addvalue::lua_l_addvalue,
    lua_l_typename::lua_l_typename, lua_rawgeti::lua_rawgeti,
  },
  macros::{getstr::getstr, lua_l_error::luaL_error},
  records::{lua_l_strbuf::LuaLStrbuf, lua_state::LuaState, lua_table::LuaTable},
};

/// # Safety
///
/// `l`/`t` 为存活 `LuaState` 与可读 LuaTable（`t` 允许为 null，对应 C++ 空表慢路径），
/// `b` 为其上初始化的缓冲。
pub(crate) unsafe fn addfield(l: *mut LuaState, b: &mut LuaLStrbuf, i: i32, t: *mut LuaTable) {
  // Safety: 契约保证 `t` 非空分支的 array 段可读，拼接写入经 addlstring 缓冲协议扩展
  unsafe {
    // C++ does `cast_to(unsigned, i - 1)` here; for i = INT_MIN the `i - 1`
    // is signed-overflow UB upstream (ltablib.cpp:232). wrapping_sub matches
    // the two's-complement value C++ relies on: it wraps to INT_MAX, fails the
    // `< sizearray` bound, and falls through to the rawgeti slow path.
    if !t.is_null()
      && (i.wrapping_sub(1) as u32) < (*t).sizearray as u32
      && (*(*t).array.add(i.wrapping_sub(1) as usize)).is_string()
    {
      let ts = (*(*t).array.add((i - 1) as usize)).as_string_ptr();
      lua_l_addlstring(
        b,
        from_raw_parts(getstr(ts) as *const u8, (*ts).len as usize),
      );
    } else {
      let tt = lua_rawgeti(l, 1, i);
      if tt != LuaType::String as i32 && tt != LuaType::Number as i32 {
        let tn = cstr_cow(lua_l_typename(l, -1));
        luaL_error!(
          l,
          "invalid value ({}) at index {} in table for 'concat'",
          tn,
          i
        );
      }
      lua_l_addvalue(b);
    }
  }
}
