use core::ffi::c_int;

use ulua_vm::{records::lua_state::LuaState, type_aliases::lua_c_function::LuaCFunction};

use crate::common::functions::{
  multiple_yields::multiple_yields,
  multiple_yields_continuation::multiple_yields_continuation,
  multiple_yields_with_nested_call::multiple_yields_with_nested_call,
  multiple_yields_with_nested_call_continuation::multiple_yields_with_nested_call_continuation,
  passthrough_call::passthrough_call,
  passthrough_call_arg_reuse::passthrough_call_arg_reuse,
  passthrough_call_arg_reuse_continuation::passthrough_call_arg_reuse_continuation,
  passthrough_call_continuation::passthrough_call_continuation,
  passthrough_call_more_results::passthrough_call_more_results,
  passthrough_call_more_results_continuation::passthrough_call_more_results_continuation,
  passthrough_call_varadic::passthrough_call_varadic,
  passthrough_call_varadic_continuation::passthrough_call_varadic_continuation,
  passthrough_call_with_state::passthrough_call_with_state,
  passthrough_call_with_state_continuation::passthrough_call_with_state_continuation,
  pcall_then_x_call::pcall_then_x_call,
  pcall_then_x_call_continuation::pcall_then_x_call_continuation,
  safe_api::{pushcclosurek, state_mut},
  single_yield::single_yield,
  single_yield_continuation::single_yield_continuation,
};

/// 注册无 upvalue 的全局 C 函数（名称同时用作 closure debugname 与全局名）。
fn push_global_fn(
  l: *mut LuaState,
  name: &'static [u8],
  f: LuaCFunction,
  cont: Option<unsafe extern "C-unwind" fn(*mut LuaState, i32) -> i32>,
) {
  pushcclosurek(l, f, Some(name), 0, cont);
  state_mut(l).set_global_bytes(name.strip_suffix(b"\0").unwrap_or(name));
}

/// K 续体槽位类型（与 [`push_global_fn`] 的 `cont` 参数同形）。
type Continuation = Option<unsafe extern "C-unwind" fn(*mut LuaState, i32) -> i32>;

/// 无 upvalue 全局 C 函数注册表：(全局名, C 函数, K 续体)。数据本身安全，
/// 注册动作留在下方窄 unsafe 调用里，逐条等价于原顺序调用序列。
const GLOBAL_FNS: &[(&[u8], LuaCFunction, Continuation)] = &[
  (
    b"singleYield\0",
    Some(single_yield),
    Some(single_yield_continuation),
  ),
  (
    b"multipleYields\0",
    Some(multiple_yields),
    Some(multiple_yields_continuation),
  ),
  (
    b"multipleYieldsWithNestedCall\0",
    Some(multiple_yields_with_nested_call),
    Some(multiple_yields_with_nested_call_continuation),
  ),
  (
    b"passthroughCall\0",
    Some(passthrough_call),
    Some(passthrough_call_continuation),
  ),
  (
    b"passthroughCallMoreResults\0",
    Some(passthrough_call_more_results),
    Some(passthrough_call_more_results_continuation),
  ),
  (
    b"passthroughCallArgReuse\0",
    Some(passthrough_call_arg_reuse),
    Some(passthrough_call_arg_reuse_continuation),
  ),
  (
    b"passthroughCallVaradic\0",
    Some(passthrough_call_varadic),
    Some(passthrough_call_varadic_continuation),
  ),
  (
    b"passthroughCallWithState\0",
    Some(passthrough_call_with_state),
    Some(passthrough_call_with_state_continuation),
  ),
];

/// pcallThenCall/pcallThenPcall：带一个 integer upvalue 的变体注册表
/// （全局名, C 函数, upvalue 初值, K 续体）。
const INT_UPVALUE_FNS: &[(&[u8], LuaCFunction, c_int, Continuation)] = &[
  (
    b"pcallThenCall\0",
    Some(pcall_then_x_call),
    0,
    Some(pcall_then_x_call_continuation),
  ),
  (
    b"pcallThenPcall\0",
    Some(pcall_then_x_call),
    1,
    Some(pcall_then_x_call_continuation),
  ),
];

/// 注册带一个 integer upvalue 的全局 C 函数（upvalue 先压栈、闭包计 1）。
///
/// 栈序列与逐条展开的原写法一致；表项均为 'static 指针/函数项。
fn push_global_int_fn(
  l: *mut LuaState,
  name: &'static [u8],
  f: LuaCFunction,
  upvalue: c_int,
  cont: Continuation,
) {
  state_mut(l).push_integer(upvalue);
  pushcclosurek(l, f, Some(name), 1, cont);
  state_mut(l).set_global_bytes(name.strip_suffix(b"\0").unwrap_or(name));
}

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_c_yield_setup(l: *mut LuaState) {
  for (name, f, cont) in GLOBAL_FNS {
    push_global_fn(l, name, *f, *cont);
  }

  for (name, f, upvalue, cont) in INT_UPVALUE_FNS {
    push_global_int_fn(l, name, *f, *upvalue, *cont);
  }
}
