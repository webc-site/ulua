//! int64_div/int64_idiv 共享的除零与 i64 溢出检查（cpp lmathlib 同款语义）

use crate::{macros::lua_l_error::luaL_error, records::lua_state::LuaState};

/// 除零错误单点（int64 除余族 udiv/urem/rem/mod/div/idiv 共用文案收口）：
/// 消息逐字为 "division by zero"，与收口前各调用点一致；`b == 0` 在 i64 域与其
/// u64 重解读下等价（0 的两种形态同值），调用方传 `b as u64` 无判别漂移。
///
/// # Safety
/// `l` 必须是有效且存活的 `lua_State` 指针。
#[inline]
pub(crate) unsafe fn check_nonzero_divisor(l: *mut LuaState, b: u64) {
  // Safety: 契约保证 `l` 为存活调用帧，b 为栈上取回的纯数值，报错经 luaL_error 不返回
  unsafe {
    if b == 0 {
      luaL_error!(l, "division by zero");
    }
  }
}

/// 除零与 `i64::MIN / -1` 溢出检查：不合法直接抛 Lua 错误（与 cpp `luaL_error` 一致）。
///
/// # Safety
/// `l` 必须是有效且存活的 `lua_State` 指针。
#[inline]
pub(crate) unsafe fn check_div_args_64(l: *mut LuaState, a: i64, b: i64) {
  // Safety: 契约保证 `l` 为存活调用帧且实参 1/2 栈槽可读，块内取回的除数 TValue 比较仅在帧界内
  unsafe {
    check_nonzero_divisor(l, b as u64);
    if a == i64::MIN && b == -1 {
      luaL_error!(l, "integer overflow");
    }
  }
}
