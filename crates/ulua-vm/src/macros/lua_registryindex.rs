pub use crate::macros::luai_maxcstack::LUAI_MAXCSTACK;

/// 伪索引：指向注册表（`LUA_REGISTRYINDEX`）
pub const LUA_REGISTRYINDEX: i32 = -LUAI_MAXCSTACK - 2000;
