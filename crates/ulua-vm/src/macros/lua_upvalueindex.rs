pub use crate::macros::lua_globalsindex::LUA_GLOBALSINDEX;

/// C 闭包第 `i` 个 upvalue 的伪索引（`lua_upvalueindex`）
#[inline(always)]
pub const fn lua_upvalueindex(i: i32) -> i32 {
  LUA_GLOBALSINDEX - i
}
