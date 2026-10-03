use core::slice::from_raw_parts;

use crate::{
  functions::{
    lua_l_checklstring::lua_l_checklstring,
    utf_8_decode::{is_cont_byte, utf_8_decode},
  },
  macros::{lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 的存活/独占前提已由 `&mut` 接收者类型承载（r16-v41 收形），屏障仍保留是因为体内有一处
/// 真实裸操作：`lua_l_checklstring` 返回覆盖 `[0,len]`（含 NUL 终止符）的 `*const c_char`，随后
/// `from_raw_parts` 依该契约把裸指针重建成 `bytes` 目标窗（第 `len` 处恒为 NUL，tstring 布局保证），
/// 窗内解码/续字节判定均在有界读内完成。另：栈 index 1 须为字符串（`lua_l_checklstring` 非串即抛错
/// 发散）、index 2 为上一步游标整数；解码越界或非法 UTF-8 经 `luaL_error` 抛错，须在可抛错的受保护帧内调用。
/// cpp `lutf8lib.cpp:240`。
pub unsafe fn iter_aux(l: &mut LuaState) -> i32 {
  unsafe {
    let mut len: usize = 0;
    let s = lua_l_checklstring(l, 1, &mut len);
    // Lua 字符串恒有 NUL 终止（utf_8_decode 的入约模型）：切片覆盖到含终止符，
    // 之后所有解码/续字节判定均在有界读内完成
    // SAFETY: s 指向 len 字节的 Lua 串数据，第 len 处恒为 NUL 终止符（tstring 布局保证）
    let bytes = from_raw_parts(s as *const u8, len + 1);
    let mut n = l.to_integer(2).unwrap_or(0) - 1;

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
      // cpp `if (!code || iscont(...)) error`：两臂判定收敛为 match 守卫，
      // 失败经 luaL_error(!) 抛出不返回，消除哨兵回退值（None 臂不求值守卫，
      // 与原 `||` 短路同序）
      let code = match code {
        Some(code) if !is_cont_byte(bytes[n as usize + step]) => code,
        _ => luaL_error!(l.as_mut_ptr(), "invalid UTF-8 code"),
      };
      l.push_integer(n + 1);
      l.push_integer(code as i32);
      2
    }
  }
}

lua_lib_fn!(pub fn iter_aux @ref, iter_aux_arm);
