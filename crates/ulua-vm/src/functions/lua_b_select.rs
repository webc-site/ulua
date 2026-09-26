use core::ffi::c_char;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_gettop::lua_gettop, lua_l_checkinteger::lua_l_checkinteger,
    lua_pushinteger::lua_pushinteger, lua_type::lua_type,
  },
  macros::{lua_l_argcheck::luaL_argcheck, lua_lib_fn::lua_lib_fn, lua_tostring::lua_tostring},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`lua_gettop` 取实参数 n（须 ≥1）；若索引 1 为字符串则
/// `lua_tostring` 读其首字节判 '#'（该串槽存活）；否则 `lua_l_checkinteger(l,1)` 要求可转整数，
/// `luaL_argcheck` 校验归一化下标 1≤i 越界即抛错回退；走 '#' 分支时 `lua_pushinteger` 需 `(*l).top` 后 ≥1 空槽。
/// cpp VM/src/lbaselib.cpp:265
pub(crate) unsafe fn lua_b_select(l: *mut LuaState) -> i32 {
  unsafe {
    let n = lua_gettop(l);
    let first_type = lua_type(l, 1);
    if first_type == LuaType::String as i32 {
      let str_ptr = lua_tostring!(l, 1);
      let first_char = *str_ptr;
      if first_char == b'#' as c_char {
        lua_pushinteger(l, n - 1);
        return 1;
      }
    }

    let i = lua_l_checkinteger(l, 1);
    let i = if i < 0 {
      n + i
    } else if i > n {
      n
    } else {
      i
    };

    luaL_argcheck!(l, 1 <= i, 1, "index out of range");
    n - i
  }
}

lua_lib_fn!(pub(crate) fn lua_b_select, lua_b_select_arm);
