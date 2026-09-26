use crate::{
  functions::{lua_isnumber::lua_isnumber, lua_rawgetfield::lua_rawgetfield_bytes},
  macros::{lua_l_error::luaL_error, lua_pop::lua_pop, lua_tointeger::lua_tointeger},
  records::lua_state::LuaState,
};

/// `key` 为纯 Rust 字节切片（如 `b"sec"`），直接传给 `lua_rawgetfield_bytes`。
///
/// # Safety
///
/// `L` 必须指向存活 `LuaState` 且表位于约定的相对索引处，键/值槽按各参数约定可读/可写。
pub(crate) unsafe fn getfield(l: *mut LuaState, key: &[u8], d: i32) -> i32 {
  // Safety: 契约保证 `L` 栈顶相对索引处为可读表，pushstr! 与 rawget 前后栈操作不越帧界
  unsafe {
    lua_rawgetfield_bytes(l, -1, key);

    if lua_isnumber(l, -1) != 0 {
      // `lua_tointeger!` 即 `i32`，无需再 cast
      let res = lua_tointeger!(l, -1);
      lua_pop(l, 1);
      res
    } else {
      if d < 0 {
        let key_text = String::from_utf8_lossy(key);
        luaL_error!(l, "field '{}' missing in date table", key_text);
      }
      lua_pop(l, 1);
      d
    }
  }
}
