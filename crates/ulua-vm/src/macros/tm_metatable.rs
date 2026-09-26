//! 受保护元表字段名 `__metatable`（cpp lua_getmetatable 的 __metatable 拦截）。
//!
//! NUL 结尾字节串面向 `luaL_getmetafield` 等 `*const c_char` 契约——不引入 `CStr` 类型。

/// NUL 结尾字段名（`luaL_getmetafield` 等 `*const c_char` 契约用）。
pub const TM_METATABLE: &[u8] = b"__metatable\0";
