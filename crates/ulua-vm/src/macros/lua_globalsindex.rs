pub use crate::macros::luai_maxcstack::LUAI_MAXCSTACK;

/// 伪索引：指向全局表（`LUA_GLOBALSINDEX`）
pub const LUA_GLOBALSINDEX: i32 = -LUAI_MAXCSTACK - 2002;
