//! Rust 字符串压入 Lua 栈的唯一收口。

/// 把 `s` 原样压入 VM 栈。
///
/// Rust `str`/`String` 不带 NUL 结尾，因此必须走长度版 `lua_pushlstring`；
/// 各处手写 `as_ptr() as *const c_char, len()` 的指针往返收敛到这一处。
///
/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`；
/// `s` 的字节在调用期间存活（借自调用方的 arena/栈对象，本函数不接管所有权）。
use crate::{records::arena_handle::alias};
use ulua_vm::records::lua_state::LuaState;
pub(crate) unsafe fn push_string(l: &mut LuaState, s: &str) {
  l.push_bytes(s.as_bytes())
}
