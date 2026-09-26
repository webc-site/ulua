use crate::{
  functions::{
    lua_l_checknumber::lua_l_checknumber, lua_pushinteger_64::lua_pushinteger_64,
    lua_pushnil::lua_pushnil,
  },
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `lua_State` 且栈 index 1 为可转 `double` 的值（`lua_l_checknumber` 读槽并可抛错），
/// `lua_pushinteger64`/`pushnil` 可能扩栈，须在受保护帧内由 C 侧调入。cpp `lintlib.cpp:19`。
pub unsafe extern "C-unwind" fn int64_create(l: *mut LuaState) -> i32 {
  unsafe {
    let x = lua_l_checknumber(l, 1);

    // C++: if (x >= -9223372036854775808.0 && x < 9223372036854775808.0)
    // These constants are exactly -2^63 and 2^63.
    if (-9223372036854775808.0..9223372036854775808.0).contains(&x) {
      let val = x as i64;

      // C++: if (((double)l) == x)
      if (val as f64) == x {
        lua_pushinteger_64(l, val);
        return 1;
      }
    }

    lua_pushnil(l);
    1
  }
}
