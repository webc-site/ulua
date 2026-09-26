use core::{ffi::c_char, ptr::null_mut};

use ulua_common::{functions::is_c_space::is_c_space, strtoull_shim::rust_strtoull};

use crate::{
  functions::{
    lua_l_checkany::lua_l_checkany, lua_l_optinteger::lua_l_optinteger, lua_pushnil::lua_pushnil,
    lua_pushnumber::lua_pushnumber, lua_tonumberx::lua_tonumberx,
  },
  macros::{lua_l_argcheck::luaL_argcheck, lua_l_checkstring::luaL_checkstring},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState`；栈 index 1 为待转值、index 2 为可选进制（`luaL_optinteger`/`checkany`/
/// `checkstring` 读槽，进制越界经 `luaL_argcheck` 抛错）；base16 分支把 index 1 的 NUL 结尾 C 串交给
/// `rust_strtoull`，须在受保护帧内调用。cpp `lbaselib.cpp:39`。
pub(crate) unsafe extern "C-unwind" fn lua_b_tonumber(l: *mut LuaState) -> i32 {
  unsafe {
    let base = lua_l_optinteger(l, 2, 10);

    if base == 10 {
      // standard conversion
      if let Some(n) = lua_tonumberx(l, 1) {
        lua_pushnumber(l, n);
        return 1;
      }
      lua_l_checkany(l, 1); // error if we don't have any argument
    } else {
      let s1 = luaL_checkstring!(l, 1);
      luaL_argcheck!(l, (2..=36).contains(&base), 2, "base out of range");

      let mut s2: *mut c_char = null_mut();
      let n = rust_strtoull(s1, &mut s2, base as u32);

      if s1 != s2 {
        // at least one valid digit?
        while is_c_space(*s2 as u8) {
          s2 = s2.add(1);
        } // skip trailing spaces

        if *s2 == b'\0' as c_char {
          // no invalid trailing characters?
          lua_pushnumber(l, n as f64);
          return 1;
        }
      }
    }

    lua_pushnil(l); // else not a number
    1
  }
}
