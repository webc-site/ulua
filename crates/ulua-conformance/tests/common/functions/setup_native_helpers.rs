use core::ffi::c_int;

use ulua_code_gen::functions::luau_codegen_supported::luau_codegen_supported;
use ulua_vm::{
  functions::{
    lua_g_isnative::lua_g_isnative, lua_pushboolean::lua_pushboolean,
    lua_pushcclosurek::lua_pushcclosurek,
  },
  macros::lua_setglobal::lua_setglobal,
  records::lua_state::LuaState,
  type_aliases::lua_c_function::LuaCFunction,
};

use crate::common::functions::{cstr::cstr, run_conformance::codegen};
unsafe extern "C-unwind" fn is_native(l: *mut LuaState) -> c_int {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    lua_pushboolean(l, lua_g_isnative(l, 1));
    1
  }
}

unsafe extern "C-unwind" fn is_native_if_supported(l: *mut LuaState) -> c_int {
  // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`l` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
  unsafe {
    if !codegen() || luau_codegen_supported() == 0 {
      lua_pushboolean(l, 1);
    } else {
      lua_pushboolean(l, lua_g_isnative(l, 1));
    }

    1
  }
}
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn setup_native_helpers(l: *mut LuaState) {
  // 两个闭包指针是纯 Rust 形式转换（`as` 同签名 upcast），不触碰 `l`。
  let is_native_fn: LuaCFunction =
    Some(is_native as unsafe extern "C-unwind" fn(*mut LuaState) -> c_int);
  let is_native_if_supported_fn: LuaCFunction =
    Some(is_native_if_supported as unsafe extern "C-unwind" fn(*mut LuaState) -> c_int);

  // Safety: `l` 为本用例存活的 LuaState；用 NUL 结尾静态名建闭包并登记为全局 `is_native`。
  unsafe {
    lua_pushcclosurek(l, is_native_fn, cstr(b"is_native\0"), 0, None);
    lua_setglobal(l, cstr(b"is_native\0"));
  }

  // Safety: 同上——登记为全局 `is_native_if_supported`。
  unsafe {
    lua_pushcclosurek(
      l,
      is_native_if_supported_fn,
      cstr(b"is_native_if_supported\0"),
      0,
      None,
    );
    lua_setglobal(l, cstr(b"is_native_if_supported\0"));
  }
}
