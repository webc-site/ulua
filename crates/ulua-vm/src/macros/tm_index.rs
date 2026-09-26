//! 元方法字段名 `__index`（cpp TM_INDEX 对应字符串）。
//!
//! NUL 结尾字节串面向 `lua_setfield` 等 `*const c_char` 契约——不引入 `CStr` 类型。

/// NUL 结尾字段名（`lua_setfield` 等 `*const c_char` 契约用）。
pub const TM_INDEX: &[u8] = b"__index\0";
