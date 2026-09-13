use ulua_vm::{
  functions::{lua_pushcclosurek::lua_pushcclosurek, lua_pushinteger::lua_pushinteger},
  macros::lua_setglobal::lua_setglobal,
  records::lua_state::lua_State,
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
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_c_yield_setup(l: *mut lua_State) {
  unsafe {
    lua_pushcclosurek(
      l,
      Some(single_yield),
      c"singleYield".as_ptr(),
      0,
      Some(single_yield_continuation),
    );
    lua_setglobal(l, c"singleYield".as_ptr());

    lua_pushcclosurek(
      l,
      Some(multiple_yields),
      c"multipleYields".as_ptr(),
      0,
      Some(multiple_yields_continuation),
    );
    lua_setglobal(l, c"multipleYields".as_ptr());

    lua_pushcclosurek(
      l,
      Some(multiple_yields_with_nested_call),
      c"multipleYieldsWithNestedCall".as_ptr(),
      0,
      Some(multiple_yields_with_nested_call_continuation),
    );
    lua_setglobal(l, c"multipleYieldsWithNestedCall".as_ptr());

    lua_pushcclosurek(
      l,
      Some(passthrough_call),
      c"passthroughCall".as_ptr(),
      0,
      Some(passthrough_call_continuation),
    );
    lua_setglobal(l, c"passthroughCall".as_ptr());

    lua_pushcclosurek(
      l,
      Some(passthrough_call_more_results),
      c"passthroughCallMoreResults".as_ptr(),
      0,
      Some(passthrough_call_more_results_continuation),
    );
    lua_setglobal(l, c"passthroughCallMoreResults".as_ptr());

    lua_pushcclosurek(
      l,
      Some(passthrough_call_arg_reuse),
      c"passthroughCallArgReuse".as_ptr(),
      0,
      Some(passthrough_call_arg_reuse_continuation),
    );
    lua_setglobal(l, c"passthroughCallArgReuse".as_ptr());

    lua_pushcclosurek(
      l,
      Some(passthrough_call_varadic),
      c"passthroughCallVaradic".as_ptr(),
      0,
      Some(passthrough_call_varadic_continuation),
    );
    lua_setglobal(l, c"passthroughCallVaradic".as_ptr());

    lua_pushcclosurek(
      l,
      Some(passthrough_call_with_state),
      c"passthroughCallWithState".as_ptr(),
      0,
      Some(passthrough_call_with_state_continuation),
    );
    lua_setglobal(l, c"passthroughCallWithState".as_ptr());

    lua_pushinteger(l, 0);
    lua_pushcclosurek(
      l,
      Some(pcall_then_x_call),
      c"pcallThenCall".as_ptr(),
      1,
      Some(pcall_then_x_call_continuation),
    );
    lua_setglobal(l, c"pcallThenCall".as_ptr());

    lua_pushinteger(l, 1);
    lua_pushcclosurek(
      l,
      Some(pcall_then_x_call),
      c"pcallThenPcall".as_ptr(),
      1,
      Some(pcall_then_x_call_continuation),
    );
    lua_setglobal(l, c"pcallThenPcall".as_ptr());
  }
}
