//! 错误处理中出错消息（cpp ERRERRMSG 宏同款）。
//!
//! `LUA_ERRERRMSG_STR` 面向 Rust 侧（无 NUL 语义）。

/// 不含 NUL 的消息文本（Rust 侧展示用）。
pub const LUA_ERRERRMSG_STR: &str = "error in error handling";
