use core::ffi::c_char;

use crate::{
  functions::{
    lua_call::lua_call, lua_l_getmetafield::lua_l_getmetafield, lua_pushvalue::lua_pushvalue,
  },
  macros::abs_index::abs_index,
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 LuaState 并处于可调用/可 GC 的受保护帧；`obj` 为 `abs_index` 归一后的合法栈索引（`lua_pushvalue`/元方法入栈读该槽），
/// `event` 须为 NUL 结尾 C 串（`lua_l_getmetafield` push 时 intern）；命中元方法后 `lua_call` 建立新帧、可抛错。
/// cpp/VM/src/laux.cpp:297 luaL_callmeta。
pub unsafe fn lua_l_callmeta(l: *mut LuaState, obj: i32, event: *const c_char) -> i32 {
  unsafe {
    let obj = abs_index(&*l, obj);
    if lua_l_getmetafield(l, obj, event) == 0 {
      return 0;
    }

    lua_pushvalue(l, obj);
    lua_call(l, 1, 1);
    1
  }
}
