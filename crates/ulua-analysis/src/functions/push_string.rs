//! Rust 字符串压入 Lua 栈的唯一收口。

use ulua_vm::records::lua_state::LuaState;
pub(crate) fn push_string(l: &mut LuaState, s: &str) {
  l.push_bytes(s.as_bytes())
}
