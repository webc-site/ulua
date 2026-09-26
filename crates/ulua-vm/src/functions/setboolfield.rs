use crate::{
  functions::{lua_pushboolean::lua_pushboolean, lua_setfield::lua_setfield_bytes},
  records::lua_state::LuaState,
};

/// `key` 为纯 Rust 字节切片（如 `b"isdst"`），直接传给 `lua_setfield_bytes`。
///
/// # Safety
///
/// `L` 必须指向存活 `LuaState` 且表位于约定的相对索引处，键/值槽按各参数约定可读/可写。
pub(crate) unsafe fn setboolfield(l: *mut LuaState, key: &[u8], value: i32) {
  if value < 0 {
    return;
  }

  // Safety: 契约保证 `L` 栈顶相对索引处为可写表，rawset 压入的键值槽位于当前帧界内
  unsafe {
    lua_pushboolean(l, value);

    lua_setfield_bytes(l, -2, key);
  }
}
