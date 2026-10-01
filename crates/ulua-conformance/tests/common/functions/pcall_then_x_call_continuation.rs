use ulua_common::macros::luau_assert::LUAU_ASSERT;
use ulua_vm::{
  enums::lua_status::LuaStatus,
  macros::{lua_multret::LUA_MULTRET, lua_upvalueindex::lua_upvalueindex},
  records::lua_state::LuaState,
};

use crate::common::functions::safe_api::{
  callyieldable, l_checkstack, pcallyieldable, state_mut,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn pcall_then_x_call_continuation(
  l: *mut LuaState,
  status: i32,
) -> i32 {
  // 预留 1 个栈位（失败时按 cpp 抛栈溢出错误）。
  l_checkstack(l, 1, "pcallThenCallContinuation");

  // `lua_upvalueindex(1)` 指向本闭包的 pcall 变体标记整数；`lua_tointeger!`
  // 不可转换时返回 0，与 cpp 同语义。
  let pcall_variant = state_mut(l).to_integer(lua_upvalueindex(1)).unwrap_or(0);
  // `check_integer` 校验参数 3 为整数（否则抛 Lua 错误）。
  let state = state_mut(l).check_integer(3);

  if state == 0 {
    // status 非 Ok 时先压 -1 再 replace 到 4 号位，否则直接 replace
    // ——两步相邻，栈平衡与 cpp 一致。
    if status != LuaStatus::Ok as i32 {
      state_mut(l).push_integer(-1);
      state_mut(l).replace(4);
    } else {
      state_mut(l).replace(4);
    }

    // 把 state 标记为 1 后写回 3 号位。
    state_mut(l).push_integer(1);
    state_mut(l).replace(3);

    // 2 号位是待执行的第二个函数值，pcall 变体决定受保护与否，
    // LUA_MULTRET 按 cpp 回传全部结果。
    state_mut(l).push_value(2); // call second function
    if pcall_variant != 0 {
      pcallyieldable(l, 0, LUA_MULTRET, 0)
    } else {
      callyieldable(l, 0, LUA_MULTRET)
    }
  } else {
    // 参数 4 为整数乘子（否则抛 Lua 错误）。
    let multiplier = state_mut(l).check_integer(4);
    // status 非 Ok 时按 cpp 断言只可能是 pcall 变体（值记 -1）；否则读栈顶整数
    //（`lua_tointeger!` 不可转换时返回 0）。
    let value = if status != LuaStatus::Ok as i32 {
      LUAU_ASSERT!(pcall_variant != 0);
      -1
    } else {
      state_mut(l).to_integer(-1).unwrap_or(0)
    };

    // 压入纯算术结果作为续体返回值。
    state_mut(l).push_integer(multiplier * value);
    1
  }
}
