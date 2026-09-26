//! 内存不足错误消息（cpp MEMERRMSG 宏同款）。
//!
//! `STR` 面向 Rust 侧（无 NUL 语义），`BYTES` 带 NUL 结尾面向 VM 的
//! `lua_pushstring` 等指针契约——不引入 `CStr` 类型。

/// 不含 NUL 的消息文本（Rust 侧展示/缓存用）。
pub const LUA_MEMERRMSG_STR: &str = "not enough memory";

/// NUL 结尾字节串（`lua_pushstring` 等 `*const c_char` 契约用）。
pub const LUA_MEMERRMSG: &[u8] = b"not enough memory\0";
