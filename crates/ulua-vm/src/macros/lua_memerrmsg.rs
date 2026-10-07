//! 内存不足错误消息（cpp MEMERRMSG 宏同款）。
//!
//! `LUA_MEMERRMSG_STR` 面向 Rust 侧（无 NUL 语义）。

/// 不含 NUL 的消息文本（Rust 侧展示/缓存用）。
pub const LUA_MEMERRMSG_STR: &str = "not enough memory";
