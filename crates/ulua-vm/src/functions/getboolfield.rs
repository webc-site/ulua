use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_rawgetfield::lua_rawgetfield_bytes, lua_toboolean::lua_toboolean, lua_type::lua_type,
  },
  macros::lua_pop::lua_pop,
  records::lua_state::LuaState,
};

/// `key` 为纯 Rust 字节切片（如 `b"isdst"`），直接传给 `lua_rawgetfield_bytes`。
///
/// # Safety
///
/// `L` 必须指向存活 `LuaState` 且表位于约定的相对索引处，键/值槽按各参数约定可读/可写。
pub(crate) unsafe fn getboolfield(l: *mut LuaState, key: &[u8]) -> i32 {
  // Safety: 契约保证 `L` 栈顶相对索引处为可读表，rawget 探测与布尔读取在当前帧栈界内完成
  unsafe {
    lua_rawgetfield_bytes(l, -1, key);

    let is_nil = lua_type(l, -1) == (LuaType::Nil as i32);

    let res: i32 = if is_nil { -1 } else { lua_toboolean(l, -1) };

    lua_pop(l, 1);
    res
  }
}
