//! int64_div/int64_idiv 共享的除零与 i64 溢出检查（cpp lmathlib 同款语义）

use crate::{macros::lua_l_error::luaL_error, records::lua_state::LuaState};

/// 除零错误单点（int64 除余族 udiv/urem/rem/mod/div/idiv 共用文案收口）：
/// 消息逐字为 "division by zero"，与收口前各调用点一致；`b == 0` 在 i64 域与其
/// u64 重解读下等价（0 的两种形态同值），调用方传 `b as u64` 无判别漂移。
///
/// 本函数为 safe：报错路径经 `luaL_error` 抛 Lua 错误不返回，`b` 为栈上取回的
/// 纯数值，`l` 以 `&mut` 借用形式满足「存活调用帧」契约。
#[inline]
pub(crate) fn check_nonzero_divisor(l: &mut LuaState, b: u64) {
  if b == 0 {
    // SAFETY: `l` 为借用形式的存活调用帧（&mut 保证有效且独占），luaL_error 抛错不返回。
    unsafe { luaL_error!(l.as_mut_ptr(), "division by zero") };
  }
}

/// 除零与 `i64::MIN / -1` 溢出检查：不合法直接抛 Lua 错误（与 cpp `luaL_error` 一致）。
/// safe 的理由同 [`check_nonzero_divisor`]。
#[inline]
pub(crate) fn check_div_args_64(l: &mut LuaState, a: i64, b: i64) {
  check_nonzero_divisor(l, b as u64);
  if a == i64::MIN && b == -1 {
    // SAFETY: 同 check_nonzero_divisor。
    unsafe { luaL_error!(l.as_mut_ptr(), "integer overflow") };
  }
}
