pub use crate::macros::luai_maxcstack::LUAI_MAXCSTACK;

/// 伪索引：指向当前环境表（`LUA_ENVIRONINDEX`）
pub const LUA_ENVIRONINDEX: i32 = -LUAI_MAXCSTACK - 2001;
