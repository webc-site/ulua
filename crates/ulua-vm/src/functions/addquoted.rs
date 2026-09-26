//! Source: `VM/src/lstrlib.cpp:884`
//!
//! Helper for `string.format("%q", s)` — append `s` to the buffer as a quoted,
//! escapable string literal: wrap in `"`, backslash-escape `"`/`\`/newline,
//! emit `\r` and `\000` for CR and NUL, pass everything else through.

use core::{ffi::c_char, slice::from_raw_parts};

use crate::{
  functions::{
    lua_l_addchar::lua_l_addchar, lua_l_addlstring::lua_l_addlstring,
    lua_l_checklstring::lua_l_checklstring, lua_l_prepbuffsize::lua_l_prepbuffsize,
  },
  records::{lua_l_strbuf::LuaLStrbuf, lua_state::LuaState},
};

/// # Safety
///
/// `l` 须为存活调用帧，栈槽 `arg` 为可读串值（`lua_l_checklstring` 前置条件），
/// 该串在本次调用期间存活。
pub(crate) unsafe fn addquoted(l: *mut LuaState, b: &mut LuaLStrbuf, arg: i32) {
  // Safety: 契约保证 s[..len] 为栈槽 `arg` 串 payload 的可读字节区（checklstring 返回
  // 长度即 payload 长，不含 NUL），切片后逐字节转义拼接不越出该串长度
  unsafe {
    let mut len: usize = 0;
    let s = lua_l_checklstring(l, arg, &mut len);
    let bytes = from_raw_parts(s as *const u8, len);

    lua_l_prepbuffsize(b, len + 2);

    lua_l_addchar(b, b'"' as c_char);
    for &c in bytes {
      match c {
        b'"' | b'\\' | b'\n' => {
          lua_l_addchar(b, b'\\' as c_char);
          lua_l_addchar(b, c as c_char);
        }
        b'\r' => {
          lua_l_addlstring(b, b"\\r");
        }
        b'\0' => {
          lua_l_addlstring(b, b"\\000");
        }
        _ => {
          lua_l_addchar(b, c as c_char);
        }
      }
    }
    lua_l_addchar(b, b'"' as c_char);
  }
}
