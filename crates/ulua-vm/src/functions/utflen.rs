use core::slice::from_raw_parts;

use crate::{
  functions::{
    lua_l_checklstring::lua_l_checklstring, lua_l_optinteger::lua_l_optinteger,
    lua_pushinteger::lua_pushinteger, lua_pushnil::lua_pushnil, u_posrelat::u_posrelat,
    utf_8_decode::utf_8_decode,
  },
  macros::lua_l_argcheck::luaL_argcheck,
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn utflen(l: *mut LuaState) -> i32 {
  unsafe {
    let mut n: i32 = 0;
    let mut len: usize = 0;
    let s = lua_l_checklstring(l, 1, &mut len);
    // Lua 字符串恒有 NUL 终止（utf_8_decode 的入约模型）：切片覆盖到含终止符，
    // 循环游标改字节下标，取代 cpp 的裸指针推进与差值回算
    // Safety: s 指向 len 字节的 Lua 串数据，第 len 处恒为 NUL 终止符（tstring 布局保证）
    let bytes = from_raw_parts(s as *const u8, len + 1);

    let posi = u_posrelat(lua_l_optinteger(l, 2, 1), len);
    let mut posj = u_posrelat(lua_l_optinteger(l, 3, -1), len);

    luaL_argcheck!(
      l,
      1 <= posi && posi <= len as i32 + 1,
      2,
      "initial position out of string"
    );
    posj -= 1;
    luaL_argcheck!(l, posj < len as i32, 3, "final position out of string");

    let mut posi = posi - 1;

    while posi <= posj {
      let (step, code) = utf_8_decode(&bytes[posi as usize..]);
      if code.is_none() {
        lua_pushnil(l);
        lua_pushinteger(l, posi + 1);
        return 2;
      }
      // cpp `posi = s1 - s`：解码成功时 step >= 1，循环必前进
      posi += step as i32;
      n += 1;
    }

    lua_pushinteger(l, n);
    1
  }
}
