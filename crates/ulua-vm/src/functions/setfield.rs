use crate::{
  functions::{lua_pushinteger::lua_pushinteger, lua_setfield::lua_setfield_bytes},
  records::lua_state::LuaState,
};

/// `key` 为纯 Rust 字节切片（如 `b"sec"`），直接传给 `lua_setfield_bytes`。
///
/// # Safety
///
/// `L` 必须指向存活 `LuaState` 且表位于约定的相对索引处，键/值槽按各参数约定可读/可写。
pub(crate) unsafe fn setfield(l: *mut LuaState, key: &[u8], value: i32) {
  // Safety: 契约保证 `L` 栈顶相对索引处为可写表，pushvalue!/rawset 组合仅触及当前栈顶
  unsafe {
    lua_pushinteger(l, value);

    lua_setfield_bytes(l, -2, key);
  }
}
