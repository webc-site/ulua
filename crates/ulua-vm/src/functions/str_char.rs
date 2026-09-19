use core::{
  ffi::{c_char, c_int},
  ptr::null_mut,
  slice,
};

use crate::{
  functions::{
    lua_gettop::lua_gettop, lua_l_buffinitsize::lua_l_buffinitsize,
    lua_l_checkinteger::lua_l_checkinteger, lua_l_pushresultsize::lua_l_pushresultsize,
  },
  macros::{lua_l_argcheck::luaL_argcheck, uchar::uchar},
  records::lua_l_strbuf::{LUA_BUFFERSIZE, LuaLStrbuf},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_str_char"))]
pub(crate) unsafe extern "C-unwind" fn str_char(l: *mut lua_State) -> c_int {
  unsafe {
    let n = lua_gettop(l); // number of arguments

    let mut b = LuaLStrbuf {
      p: null_mut(),
      end: null_mut(),
      l: null_mut(),
      storage: null_mut(),
      buffer: [0; LUA_BUFFERSIZE],
    };
    let ptr = lua_l_buffinitsize(l, &mut b as *mut LuaLStrbuf, n as usize);

    // 输出切片 zip 参数序号：写入受切片边界约束，消除手写 offset 算术
    for (i, slot) in slice::from_raw_parts_mut(ptr, n as usize)
      .iter_mut()
      .enumerate()
    {
      let c = lua_l_checkinteger(l, i as c_int + 1);
      luaL_argcheck!(
        l,
        i32::from(uchar(c)) == c as c_int,
        i as c_int + 1,
        "invalid value"
      );

      *slot = uchar(c) as c_char;
    }
    lua_l_pushresultsize(&mut b as *mut LuaLStrbuf, n as usize);
    1
  }
}
