//! Rust 字符串压入 Lua 栈的唯一收口。

use ulua_vm::{functions::lua_pushlstring::lua_pushlstring, records::lua_state};

use crate::type_aliases::lua_state::LuaState;

/// 把 `s` 原样压入 VM 栈。
///
/// Rust `str`/`String` 不带 NUL 结尾，因此必须走长度版 `lua_pushlstring`；
/// 各处手写 `as_ptr() as *const c_char, len()` 的指针往返收敛到这一处。
///
/// # Safety
/// `l` 必须是 Lua VM 在本次原生函数调用中传入、且在该调用全程有效的 `lua_State*`；
/// `s` 的字节在调用期间存活（借自调用方的 arena/栈对象，本函数不接管所有权）。
pub(crate) unsafe fn push_string(l: *mut LuaState, s: &str) {
  // Safety: 调用方保证 `l` 存活；`s` 由 `&str` 保证字节可读且 `len` 与容量匹配，
  // `lua_pushlstring` 按长度拷入 VM 内部字符串，不要求 NUL 结尾、不保留该指针。
  unsafe { lua_pushlstring(l as *mut lua_state::LuaState, s.as_ptr().cast(), s.len()) }
}
