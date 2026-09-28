use core::{ffi::c_int, ptr::null};

use ulua_vm::{
  functions::{lua_callyieldable_impl::lua_callyieldable, lua_pushcclosurek::lua_pushcclosurek},
  records::lua_state::LuaState,
  type_aliases::{lua_c_function::LuaCFunction, lua_continuation::LuaContinuation},
};

use crate::common::functions::{
  nested_multiple_yield_helper::nested_multiple_yield_helper,
  nested_multiple_yield_helper_continuation::nested_multiple_yield_helper_continuation,
  nested_multiple_yield_helper_non_yielding::nested_multiple_yield_helper_non_yielding,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn multiple_yields_with_nested_call(l: *mut LuaState) -> c_int {
  // Safety: `l` 为本用例存活的 LuaState；截断到 2 个实参。
  unsafe { (*l).set_top(2) };
  // Safety: `l` 存活；`check_boolean` 校验参数 2 为布尔（失配按 cpp 抛 Lua 错误）。
  let nested_should_yield = unsafe { (*l).check_boolean(2) };

  // Safety: `l` 存活；压入嵌套调用的两个实参（起始计数 0 与上界 5.0）。
  unsafe {
    (*l).push_integer(0);
    (*l).push_number(5.0);
  }

  if nested_should_yield {
    let f: LuaCFunction = Some(nested_multiple_yield_helper);
    let cont: LuaContinuation = Some(nested_multiple_yield_helper_continuation);
    // Safety: `l` 存活；以 1 个上值建带续体的闭包，两个函数指针均为 `extern "C-unwind"` 桩。
    unsafe { lua_pushcclosurek(l, f, null(), 1, cont) };
  } else {
    let f: LuaCFunction = Some(nested_multiple_yield_helper_non_yielding);
    // Safety: `l` 存活；同上但无续体（非 yield 分支）。
    unsafe { lua_pushcclosurek(l, f, null(), 1, None) };
  }

  // Safety: `l` 存活且栈顶为刚压入的闭包；以 0 实参调用并取 1 个结果。
  unsafe { lua_callyieldable(l, 0, 1) }
}
