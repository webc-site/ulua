use core::slice::from_raw_parts;

use crate::{
  functions::{
    lua_l_checklstring::lua_l_checklstring, lua_l_checkstack::lua_l_checkstack,
    lua_l_optinteger::lua_l_optinteger, lua_pushinteger::lua_pushinteger, u_posrelat::u_posrelat,
    utf_8_decode::utf_8_decode,
  },
  macros::{lua_l_argcheck::luaL_argcheck, lua_l_error::luaL_error},
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn codepoint(l: *mut LuaState) -> i32 {
  unsafe {
    let mut len: usize = 0;
    let s = lua_l_checklstring(l, 1, &mut len);
    // Lua 字符串恒有 NUL 终止（utf_8_decode 的入约模型）：切片覆盖到含终止符，
    // 游标改字节下标，取代 cpp 的裸指针推进比较
    // Safety: s 指向 len 字节的 Lua 串数据，第 len 处恒为 NUL 终止符（tstring 布局保证）
    let bytes = from_raw_parts(s as *const u8, len + 1);

    let posi = u_posrelat(lua_l_optinteger(l, 2, 1), len);
    let pose = u_posrelat(lua_l_optinteger(l, 3, posi), len);

    luaL_argcheck!(l, posi >= 1, 2, "out of range");
    luaL_argcheck!(l, pose <= len as i32, 3, "out of range");

    if posi > pose {
      return 0; // empty interval; return no values
    }

    if (pose as i64 - posi as i64) >= i32::MAX as i64 {
      luaL_error!(l, "string slice too long");
    }

    let n = (pose - posi) + 1;
    lua_l_checkstack(l, n, "string slice too long");

    let mut n = 0;
    // cpp `while (s < se)`：se = s + pose，指针比较等价于下标比较 i < pose
    let mut i = (posi - 1) as usize;
    while i < pose as usize {
      let (step, code) = utf_8_decode(&bytes[i..]);
      if code.is_none() {
        luaL_error!(l, "invalid UTF-8 code");
      }
      lua_pushinteger(l, code.unwrap_or(0) as i32);
      n += 1;
      // cpp `s = next`：解码成功 step >= 1，循环必前进
      i += step;
    }

    n
  }
}
