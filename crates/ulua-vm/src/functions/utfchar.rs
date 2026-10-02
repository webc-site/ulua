use crate::{
  functions::{
    buffutfchar::buffutfchar, lua_l_addlstring::lua_l_addlstring, lua_l_buffinit::lua_l_buffinit,
    lua_l_pushresult::lua_l_pushresult, lua_pushlstring::lua_pushlstring_bytes,
  },
  macros::{lua_lib_fn::lua_lib_fn, utf_8_buffsz::UTF8BUFFSZ},
  records::{lua_l_strbuf::LuaLStrbuf, lua_state::LuaState},
};

/// # Safety
///
/// `l` 必须是正在执行的 utf8 库 C 函数帧的存活 `LuaState`：栈槽 #1..top 为整数码点
/// （`buffutfchar` 逐个校验），串构造经 `lua_l_buffinit`/`lua_l_pushresult` 的栈与 GC
/// 协议，返回值即结果串在栈上的槽数。cpp lutf8lib.cpp:162 `utfchar`。
pub unsafe fn utfchar(l: *mut LuaState) -> i32 {
  unsafe {
    let mut buff = [0u8; UTF8BUFFSZ];

    let n = (*l).get_top(); // number of arguments
    if n == 1 {
      // optimize common case of single char
      let charstr = buffutfchar(l, 1, &mut buff);
      lua_pushlstring_bytes(&mut *l, charstr);
    } else {
      let mut b = LuaLStrbuf::new();
      lua_l_buffinit(&mut *l, &mut b);
      for i in 1..=n {
        let charstr = buffutfchar(l, i, &mut buff);
        lua_l_addlstring(&mut b, charstr);
      }
      lua_l_pushresult(&mut b);
    }
    1
  }
}

lua_lib_fn!(pub fn utfchar, utfchar_arm);
