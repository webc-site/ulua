use core::slice::from_raw_parts;

use crate::{
  functions::{
    lua_l_checklstring::lua_l_checklstring,
    lua_pushinteger::lua_pushinteger,
    utf_8_decode::{is_cont_byte, utf_8_decode},
  },
  macros::{lua_l_error::luaL_error, lua_tointeger::lua_tointeger},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState`：栈 index 1 为字符串（`luaL_checklstring` 返回覆盖 `[0,len]` 含终止
/// NUL 的数据指针），index 2 为上一步游标整数；解码越界或非法 UTF-8 经 `luaL_error` 抛错，
/// 须在受保护帧内调用。cpp `lutf8lib.cpp:240`。
pub unsafe extern "C-unwind" fn iter_aux(l: *mut LuaState) -> i32 {
  unsafe {
    let mut len: usize = 0;
    let s = lua_l_checklstring(l, 1, &mut len);
    // Lua 字符串恒有 NUL 终止（utf_8_decode 的入约模型）：切片覆盖到含终止符，
    // 之后所有解码/续字节判定均在有界读内完成
    // Safety: s 指向 len 字节的 Lua 串数据，第 len 处恒为 NUL 终止符（tstring 布局保证）
    let bytes = from_raw_parts(s as *const u8, len + 1);
    let mut n = lua_tointeger!(l, 2) - 1;

    if n < 0 {
      n = 0;
    } else if n < len as i32 {
      n += 1;
      while is_cont_byte(bytes[n as usize]) {
        n += 1;
      }
    }

    if n >= len as i32 {
      0
    } else {
      let (step, code) = utf_8_decode(&bytes[n as usize..]);
      if code.is_none() || is_cont_byte(bytes[n as usize + step]) {
        luaL_error!(l, "invalid UTF-8 code");
      }
      lua_pushinteger(l, n + 1);
      lua_pushinteger(l, code.unwrap_or(0) as i32);
      2
    }
  }
}
