//! int64_div/int64_idiv 共享的除零与 i64 溢出检查（cpp lmathlib 同款语义）

use crate::{macros::lua_l_error::luaL_error, type_aliases::lua_state::LuaState};

/// 除零与 `i64::MIN / -1` 溢出检查：不合法直接抛 Lua 错误（与 cpp `luaL_error` 一致）。
///
/// # Safety
/// `l` 必须是有效且存活的 `lua_State` 指针。
#[inline]
pub(crate) unsafe fn check_div_args_64(l: *mut LuaState, a: i64, b: i64) {
  unsafe {
    if b == 0 {
      luaL_error!(l, "division by zero");
    }
    if a == i64::MIN && b == -1 {
      luaL_error!(l, "integer overflow");
    }
  }
}
