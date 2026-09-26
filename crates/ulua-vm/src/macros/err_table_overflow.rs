//! 表扩容溢出错误消息（cpp `setnodevector`/`setarrayvector`/`resize` 的 "table overflow"）。
//!
//! NUL 结尾字节串面向 `*const c_char` 契约——不引入 `CStr` 类型。

/// NUL 结尾字节串（`lua_g_runerror` 等 `*const c_char` 契约用）。
pub const ERR_TABLE_OVERFLOW: &[u8] = b"table overflow\0";
