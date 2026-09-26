use core::{ffi::c_char, slice};

use crate::{
  functions::{
    lua_gettop::lua_gettop, lua_l_buffinitsize::lua_l_buffinitsize,
    lua_l_checkinteger::lua_l_checkinteger, lua_l_pushresultsize::lua_l_pushresultsize,
  },
  macros::{lua_l_argcheck::luaL_argcheck, lua_lib_fn::lua_lib_fn, uchar::uchar},
  records::{lua_l_strbuf::LuaLStrbuf, lua_state::LuaState},
};

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`lua_gettop` 取实参数 n，`luaL_buffinitsize` 返回缓冲 `ptr` 须
/// 覆盖 n 字节写区（本函数按 from_raw_parts_mut(ptr,n) 逐槽写）；对索引 1..=n `lua_l_checkinteger`+`luaL_argcheck`
/// 校验落在 0..=255（越界抛错回退）；`luaL_pushresultsize` 提交 n 字节，需 `(*l).top` 后 ≥1 空槽；分配可触发 GC。
/// cpp VM/src/lstrlib.cpp:150
pub unsafe fn str_char(l: *mut LuaState) -> i32 {
  unsafe {
    let n = lua_gettop(l); // number of arguments

    let mut b = LuaLStrbuf::new();
    let ptr = lua_l_buffinitsize(l, &mut b as *mut LuaLStrbuf, n as usize);

    // 输出切片 zip 参数序号：写入受切片边界约束，消除手写 offset 算术
    for (i, slot) in slice::from_raw_parts_mut(ptr, n as usize)
      .iter_mut()
      .enumerate()
    {
      let c = lua_l_checkinteger(l, i as i32 + 1);
      luaL_argcheck!(l, i32::from(uchar(c)) == c, i as i32 + 1, "invalid value");

      *slot = uchar(c) as c_char;
    }
    lua_l_pushresultsize(&mut b as *mut LuaLStrbuf, n as usize);
    1
  }
}

lua_lib_fn!(pub fn str_char, str_char_arm);
