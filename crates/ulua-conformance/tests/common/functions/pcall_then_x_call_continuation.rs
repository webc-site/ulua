use ulua_common::macros::luau_assert::LUAU_ASSERT;
use ulua_vm::{
  enums::lua_status::LuaStatus,
  functions::{
    lua_callyieldable_impl::lua_callyieldable, lua_l_checkinteger::lua_l_checkinteger,
    lua_l_checkstack::lua_l_checkstack, lua_pcallyieldable::lua_pcallyieldable,
    lua_pushinteger::lua_pushinteger, lua_pushvalue::lua_pushvalue, lua_replace::lua_replace,
  },
  macros::{
    lua_multret::LUA_MULTRET, lua_tointeger::lua_tointeger, lua_upvalueindex::lua_upvalueindex,
  },
  records::lua_state::LuaState,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn pcall_then_x_call_continuation(
  l: *mut LuaState,
  status: i32,
) -> i32 {
  // Safety: `l` 为本用例存活的 LuaState；预留 1 个栈位（失败时按 cpp 抛栈溢出错误）。
  unsafe { lua_l_checkstack(l, 1, "pcallThenCallContinuation") };

  // Safety: 上一步后栈位充足；`lua_upvalueindex(1)` 指向本闭包的 pcall 变体标记整数，
  // `lua_tointeger!` 不可转换时返回 0，与 cpp 同语义。
  let pcall_variant = unsafe { lua_tointeger!(l, lua_upvalueindex(1)) };
  // Safety: `l` 存活；`lua_l_checkinteger` 校验参数 3 为整数（否则抛 Lua 错误）。
  let state = unsafe { lua_l_checkinteger(l, 3) };

  if state == 0 {
    // Safety: `l` 存活；status 非 Ok 时先压 -1 再 replace 到 4 号位，否则直接 replace
    // ——两步相邻，栈平衡与 cpp 一致。
    unsafe {
      if status != LuaStatus::Ok as i32 {
        lua_pushinteger(l, -1);
        lua_replace(l, 4);
      } else {
        lua_replace(l, 4);
      }
    }

    // Safety: `l` 存活；把 state 标记为 1 后写回 3 号位。
    unsafe {
      lua_pushinteger(l, 1);
      lua_replace(l, 3);
    }

    // Safety: `l` 存活；2 号位是待执行的第二个函数值，pcall 变体决定受保护与否，
    // LUA_MULTRET 按 cpp 回传全部结果。
    unsafe {
      lua_pushvalue(l, 2); // call second function
      if pcall_variant != 0 {
        lua_pcallyieldable(l, 0, LUA_MULTRET, 0)
      } else {
        lua_callyieldable(l, 0, LUA_MULTRET)
      }
    }
  } else {
    // Safety: `l` 存活；参数 4 为整数乘子（否则抛 Lua 错误）。
    let multiplier = unsafe { lua_l_checkinteger(l, 4) };
    // Safety: status 非 Ok 时按 cpp 断言只可能是 pcall 变体（值记 -1）；否则读栈顶整数
    // （`lua_tointeger!` 不可转换时返回 0）。
    let value = unsafe {
      if status != LuaStatus::Ok as i32 {
        LUAU_ASSERT!(pcall_variant != 0);
        -1
      } else {
        lua_tointeger!(l, -1)
      }
    };

    // Safety: `l` 存活；压入纯算术结果作为续体返回值。
    unsafe { lua_pushinteger(l, multiplier * value) }
    1
  }
}
