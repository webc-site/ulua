use core::slice::from_raw_parts;

use crate::{
  functions::{
    lua_l_checkinteger::lua_l_checkinteger, lua_l_checklstring::lua_l_checklstring,
    lua_l_optinteger::lua_l_optinteger, lua_pushinteger::lua_pushinteger, lua_pushnil::lua_pushnil,
    u_posrelat::u_posrelat, utf_8_decode::is_cont_byte,
  },
  macros::{lua_l_argcheck::luaL_argcheck, lua_l_error::luaL_error},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState` 且栈 index 1 为字符串（`luaL_checklstring` 返回覆盖 `[0,len]`
/// 含终止 NUL 的数据指针）；index 2/3 为整数，越界或初位为续字节会经 `luaL_argcheck`/`luaL_error`
/// 抛错，须在受保护帧内调用。cpp `lutf8lib.cpp:191`。
pub unsafe extern "C-unwind" fn byteoffset(l: *mut LuaState) -> i32 {
  unsafe {
    let mut len: usize = 0;
    let s = lua_l_checklstring(l, 1, &mut len);
    // Safety: 契约保证 s[..=len] 覆盖串 payload 及终止 NUL；posi 钳位后恒属 [0, len]，
    // C++ 以 iscont 读终止 NUL（非续字节）的点位由有界切片同构保留
    let bytes = from_raw_parts(s as *const u8, len + 1);
    let mut n = lua_l_checkinteger(l, 2);
    let mut posi = if n >= 0 { 1 } else { len as i32 + 1 };
    posi = u_posrelat(lua_l_optinteger(l, 3, posi), len);
    luaL_argcheck!(
      l,
      1 <= posi && posi <= len as i32 + 1,
      3,
      "position out of range"
    );
    posi -= 1;

    if n == 0 {
      // find beginning of current byte sequence
      while posi > 0 && is_cont_byte(bytes[posi as usize]) {
        posi -= 1;
      }
    } else {
      if is_cont_byte(bytes[posi as usize]) {
        luaL_error!(l, "initial position is a continuation byte");
      }
      if n < 0 {
        while n < 0 && posi > 0 {
          // find beginning of previous character：先退一格，续字节连退至序列首
          //（原 `loop { posi -= 1; if !(…) break; }` 的 do-while 形态展开）
          posi -= 1;
          while posi > 0 && is_cont_byte(bytes[posi as usize]) {
            posi -= 1;
          }
          n += 1;
        }
      } else {
        n -= 1; // do not move for 1st character
        while n > 0 && posi < len as i32 {
          // find beginning of next character：先进一格，续字节连进至下一序列首
          posi += 1;
          while is_cont_byte(bytes[posi as usize]) {
            posi += 1;
          } // (cannot pass final '\0')
          n -= 1;
        }
      }
    }

    if n == 0 {
      // did it find given character?
      lua_pushinteger(l, posi + 1);
    } else {
      // no such character
      lua_pushnil(l);
    }
    1
  }
}
