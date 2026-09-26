use core::{
  ptr::{null, null_mut},
  str::from_utf8,
};

use crate::{
  functions::{
    cstr_bytes, getthread::getthread, lua_l_optinteger::lua_l_optinteger,
    lua_l_optlstring::lua_l_optlstring, lua_l_traceback::lua_l_traceback,
  },
  macros::lua_l_argcheck::luaL_argcheck,
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向存活 `LuaState` 且所查询的调用帧/Proto/输出记录按约定存活可写。
pub(crate) unsafe extern "C-unwind" fn db_traceback(l: *mut LuaState) -> i32 {
  // Safety: 契约保证 `L1` 调用栈自 level 起可读、空帧即止，`buf`/ls 输出缓冲由调用方保证可写
  unsafe {
    let (l1, arg) = getthread(l);

    // 对应 cpp ldblib.cpp 的 luaL_optstring(L, arg + 1, NULL)，
    // 即 lua_l_optlstring 不取长度（len 传 null）
    let msg_ptr = lua_l_optlstring(l, arg + 1, null(), null_mut());
    let msg = if msg_ptr.is_null() {
      None
    } else {
      Some(from_utf8(cstr_bytes(msg_ptr)).unwrap_or(""))
    };

    let default_level = if l == l1 { 1 } else { 0 };
    let level = lua_l_optinteger(l, arg + 2, default_level);

    luaL_argcheck!(l, level >= 0, arg + 2, "level can't be negative");

    lua_l_traceback(l, l1, msg, level);

    1
  }
}
