use core::ffi::c_int;

use ulua_code_gen::functions::luau_codegen_supported::luau_codegen_supported;
use ulua_vm::{records::lua_state::LuaState, type_aliases::lua_c_function::LuaCFunction};

use crate::common::functions::{
  run_conformance::codegen,
  safe_api::{g_isnative, pushcclosurek, state_mut},
};
unsafe extern "C-unwind" fn is_native(l: *mut LuaState) -> c_int {
  state_mut(l).push_boolean(g_isnative(l, 1) != 0);
  1
}

unsafe extern "C-unwind" fn is_native_if_supported(l: *mut LuaState) -> c_int {
  if !codegen() || luau_codegen_supported() == 0 {
    state_mut(l).push_boolean(true);
  } else {
    state_mut(l).push_boolean(g_isnative(l, 1) != 0);
  }
  1
}
/// # Safety
///
/// `l` 为存活 `LuaState`（fixture 注册类型即 C ABI 钩子签名）。
pub unsafe extern "C-unwind" fn setup_native_helpers(l: *mut LuaState) {
  // 两个闭包指针是纯 Rust 形式转换（`as` 同签名 upcast），不触碰 `l`。
  let is_native_fn: LuaCFunction =
    Some(is_native as unsafe extern "C-unwind" fn(*mut LuaState) -> c_int);
  let is_native_if_supported_fn: LuaCFunction =
    Some(is_native_if_supported as unsafe extern "C-unwind" fn(*mut LuaState) -> c_int);

  // 用 NUL 结尾静态名建闭包并登记为全局 `is_native`。
  pushcclosurek(l, is_native_fn, Some(b"is_native\0"), 0, None);
  state_mut(l).set_global_str("is_native");

  // 同上——登记为全局 `is_native_if_supported`。
  pushcclosurek(
    l,
    is_native_if_supported_fn,
    Some(b"is_native_if_supported\0"),
    0,
    None,
  );
  state_mut(l).set_global_str("is_native_if_supported");
}
