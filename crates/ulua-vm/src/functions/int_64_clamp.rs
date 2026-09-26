use crate::{
  functions::{
    lua_l_checkinteger_64::lua_l_checkinteger_64, lua_pushinteger_64::lua_pushinteger_64,
  },
  macros::{lua_l_argcheck::luaL_argcheck, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于受保护帧：栈 1/2/3 号位分别经 `lua_l_checkinteger_64` 取被夹值/下界/上界
/// （非整数抛错，`mi <= mx` 由 `luaL_argcheck` 校验），结果经 `lua_pushinteger_64` 写回。cpp/VM/src/lintlib.cpp:466。
pub unsafe fn int64_clamp(l: *mut LuaState) -> i32 {
  unsafe {
    let a = lua_l_checkinteger_64(l, 1);
    let mi = lua_l_checkinteger_64(l, 2);
    let mx = lua_l_checkinteger_64(l, 3);

    luaL_argcheck!(l, mi <= mx, 3, "max must be greater than or equal to min");

    if a < mi {
      lua_pushinteger_64(l, mi);
    } else if a > mx {
      lua_pushinteger_64(l, mx);
    } else {
      lua_pushinteger_64(l, a);
    }

    1
  }
}

lua_lib_fn!(pub fn int64_clamp, int64_clamp_arm);
