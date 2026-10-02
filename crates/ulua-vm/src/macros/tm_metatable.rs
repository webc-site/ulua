//! 受保护元表字段名 `__metatable`（cpp lua_getmetatable 的 __metatable 拦截）。
//!
//! 字节切片键面向 `luaL_getmetafield` 的 bytes 核心形契约（§10：内部调用方不构造
//! NUL 结尾缓冲）——不含尾部 `\0`，不引入 C 字符串类型。

/// 元表键字段名（`lua_l_getmetafield_bytes` 等 bytes 键契约用，不含尾部 `\0`）。
pub const TM_METATABLE: &[u8] = b"__metatable";
