use core::ffi::CStr;

use ulua_vm::{
  functions::{lua_pushcclosurek::lua_pushcclosurek, lua_pushinteger::lua_pushinteger},
  macros::lua_setglobal::lua_setglobal,
  records::lua_state::lua_State,
  type_aliases::lua_c_function::LuaCfunction,
};

use crate::common::functions::{
  multiple_yields::multiple_yields, multiple_yields_continuation::multiple_yields_continuation,
  multiple_yields_with_nested_call::multiple_yields_with_nested_call,
  multiple_yields_with_nested_call_continuation::multiple_yields_with_nested_call_continuation,
  passthrough_call::passthrough_call, passthrough_call_arg_reuse::passthrough_call_arg_reuse,
  passthrough_call_arg_reuse_continuation::passthrough_call_arg_reuse_continuation,
  passthrough_call_continuation::passthrough_call_continuation,
  passthrough_call_more_results::passthrough_call_more_results,
  passthrough_call_more_results_continuation::passthrough_call_more_results_continuation,
  passthrough_call_varadic::passthrough_call_varadic,
  passthrough_call_varadic_continuation::passthrough_call_varadic_continuation,
  passthrough_call_with_state::passthrough_call_with_state,
  passthrough_call_with_state_continuation::passthrough_call_with_state_continuation,
  pcall_then_x_call::pcall_then_x_call,
  pcall_then_x_call_continuation::pcall_then_x_call_continuation, single_yield::single_yield,
  single_yield_continuation::single_yield_continuation,
};

type Continuation = Option<unsafe extern "C-unwind" fn(*mut lua_State, i32) -> i32>;

/// 注册无 upvalue 的全局 C 函数（名称同时用作 closure debugname 与全局名）。
unsafe fn push_global_fn(l: *mut lua_State, name: &CStr, f: LuaCfunction, cont: Continuation) {
  unsafe {
    lua_pushcclosurek(l, f, name.as_ptr(), 0, cont);
    lua_setglobal(l, name.as_ptr());
  }
}

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_c_yield_setup(l: *mut lua_State) {
  unsafe {
    push_global_fn(
      l,
      c"singleYield",
      Some(single_yield),
      Some(single_yield_continuation),
    );
    push_global_fn(
      l,
      c"multipleYields",
      Some(multiple_yields),
      Some(multiple_yields_continuation),
    );
    push_global_fn(
      l,
      c"multipleYieldsWithNestedCall",
      Some(multiple_yields_with_nested_call),
      Some(multiple_yields_with_nested_call_continuation),
    );
    push_global_fn(
      l,
      c"passthroughCall",
      Some(passthrough_call),
      Some(passthrough_call_continuation),
    );
    push_global_fn(
      l,
      c"passthroughCallMoreResults",
      Some(passthrough_call_more_results),
      Some(passthrough_call_more_results_continuation),
    );
    push_global_fn(
      l,
      c"passthroughCallArgReuse",
      Some(passthrough_call_arg_reuse),
      Some(passthrough_call_arg_reuse_continuation),
    );
    push_global_fn(
      l,
      c"passthroughCallVaradic",
      Some(passthrough_call_varadic),
      Some(passthrough_call_varadic_continuation),
    );
    push_global_fn(
      l,
      c"passthroughCallWithState",
      Some(passthrough_call_with_state),
      Some(passthrough_call_with_state_continuation),
    );

    // pcallThenCall/pcallThenPcall：带一个 integer upvalue 的变体
    for (name, upvalue) in [(c"pcallThenCall", 0), (c"pcallThenPcall", 1)] {
      lua_pushinteger(l, upvalue);
      lua_pushcclosurek(
        l,
        Some(pcall_then_x_call),
        name.as_ptr(),
        1,
        Some(pcall_then_x_call_continuation),
      );
      lua_setglobal(l, name.as_ptr());
    }
  }
}
