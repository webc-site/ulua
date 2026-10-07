use core::ffi::c_int;

use ulua_vm::{
  records::lua_state::LuaState,
  type_aliases::{lua_c_function::LuaCFunction, lua_continuation::LuaContinuation},
};

use crate::common::functions::{
  nested_multiple_yield_helper::nested_multiple_yield_helper,
  nested_multiple_yield_helper_continuation::nested_multiple_yield_helper_continuation,
  nested_multiple_yield_helper_non_yielding::nested_multiple_yield_helper_non_yielding,
  safe_api::{callyieldable, pushcclosurek, state_mut},
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn multiple_yields_with_nested_call(l: *mut LuaState) -> c_int {
  // 截断到 2 个实参；`check_boolean` 校验参数 2 为布尔（失配按 cpp 抛 Lua 错误）。
  state_mut(l).set_top(2);
  let nested_should_yield = state_mut(l).check_boolean(2);

  // 压入嵌套调用的两个实参（起始计数 0 与上界 5.0）。
  state_mut(l).push_integer(0);
  state_mut(l).push_number(5.0);

  if nested_should_yield {
    let f: LuaCFunction = Some(nested_multiple_yield_helper);
    let cont: LuaContinuation = Some(nested_multiple_yield_helper_continuation);
    // 以 1 个上值建带续体的闭包，两个函数指针均为 `extern "C-unwind"` 桩。
    pushcclosurek(l, f, None, 1, cont);
  } else {
    let f: LuaCFunction = Some(nested_multiple_yield_helper_non_yielding);
    // 同上但无续体（非 yield 分支）。
    pushcclosurek(l, f, None, 1, None);
  }

  // 栈顶为刚压入的闭包；以 0 实参调用并取 1 个结果。
  callyieldable(l, 0, 1)
}
