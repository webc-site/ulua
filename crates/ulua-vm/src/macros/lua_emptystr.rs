//! 空字符串（NUL 结尾字节串）。
//!
//! `BYTES` 带 NUL 结尾面向 `lua_pushliteral`/`lua_pushlstring` 等 `*const c_char`
//! 指针契约——不引入 `CStr` 类型。

/// NUL 结尾空字节串（指针契约处 `LUA_EMPTYSTR.as_ptr().cast()`）。
pub const LUA_EMPTYSTR: &[u8] = b"\0";
