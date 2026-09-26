use crate::{
  functions::{
    cstr_bytes, lua_l_optinteger::lua_l_optinteger, lua_o_str_2_l::lua_o_str_2_l,
    lua_pushinteger_64::lua_pushinteger_64, lua_pushnil::lua_pushnil,
  },
  macros::{lua_l_argcheck::luaL_argcheck, lua_l_checkstring::luaL_checkstring},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于受保护帧：栈 1 号位为字符串（`luaL_checkstring!` 返回本帧存活的 NUL 结尾指针，
/// 由 `CStr::from_ptr` 读取），2 号位可选基数经 `lua_l_optinteger`/`luaL_argcheck` 校验落在 2..=36；
/// `lua_pushinteger_64`/`lua_pushnil` 写回可分配/GC。cpp/VM/src/lintlib.cpp:37 int64_fromstring。
pub unsafe extern "C-unwind" fn int64_fromstring(l: *mut LuaState) -> i32 {
  unsafe {
    let s = luaL_checkstring!(l, 1);
    let base = lua_l_optinteger(l, 2, 10);
    luaL_argcheck!(l, (2..=36).contains(&base), 2, "base out of range");

    // Safety: luaL_checkstring 返回栈上存活字符串的 NUL 结尾指针
    match lua_o_str_2_l(cstr_bytes(s), base) {
      Some(result) => lua_pushinteger_64(l, result),
      None => lua_pushnil(l),
    }

    1
  }
}
