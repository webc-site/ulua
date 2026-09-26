use core::{ffi::c_char, slice::from_raw_parts};

use crate::{
  functions::{
    buffutfchar::buffutfchar, lua_gettop::lua_gettop, lua_l_addlstring::lua_l_addlstring,
    lua_l_buffinit::lua_l_buffinit, lua_l_pushresult::lua_l_pushresult,
    lua_pushlstring::lua_pushlstring,
  },
  macros::utf_8_buffsz::UTF8BUFFSZ,
  records::{lua_l_strbuf::LuaLStrbuf, lua_state::LuaState},
};

/// # Safety
///
/// `l` 必须是正在执行的 utf8 库 C 函数帧的存活 `LuaState`：栈槽 #1..top 为整数码点
/// （`buffutfchar` 逐个校验），串构造经 `lua_l_buffinit`/`lua_l_pushresult` 的栈与 GC
/// 协议，返回值即结果串在栈上的槽数。cpp lutf8lib.cpp:162 `utfchar`。
pub unsafe extern "C-unwind" fn utfchar(l: *mut LuaState) -> i32 {
  unsafe {
    let mut buff = [0 as c_char; UTF8BUFFSZ];

    let n = lua_gettop(l); // number of arguments
    if n == 1 {
      // optimize common case of single char
      let charstr = buffutfchar(l, 1, &mut buff);
      lua_pushlstring(l, charstr.as_ptr(), charstr.len());
    } else {
      let mut b = LuaLStrbuf::new();
      lua_l_buffinit(l, &mut b);
      for i in 1..=n {
        let charstr = buffutfchar(l, i, &mut buff);
        lua_l_addlstring(
          &mut b,
          from_raw_parts(charstr.as_ptr() as *const u8, charstr.len()),
        );
      }
      lua_l_pushresult(&mut b);
    }
    1
  }
}
