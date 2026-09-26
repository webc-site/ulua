use ulua_vm::{
  functions::lua_pushcclosurek::lua_pushcclosurek, macros::lua_setglobal::lua_setglobal,
  records::lua_state::LuaState,
};

use crate::common::functions::{
  conformance_p_call_resume_error::conformance_p_call_resume_error, cstr::cstr, cxxthrow::cxxthrow,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_p_call_setup(l: *mut LuaState) {
  // Safety: `l` 为本用例存活的 LuaState；用 NUL 结尾静态名建 `cxxthrow` 闭包并登记为全局。
  unsafe {
    lua_pushcclosurek(l, Some(cxxthrow), cstr(b"cxxthrow\0"), 0, None);
    lua_setglobal(l, cstr(b"cxxthrow\0"));
  }

  // Safety: 同上——登记 `resumeerror` 闭包（桩为 `extern "C-unwind"`）。
  unsafe {
    lua_pushcclosurek(
      l,
      Some(conformance_p_call_resume_error),
      cstr(b"resumeerror\0"),
      0,
      None,
    );
    lua_setglobal(l, cstr(b"resumeerror\0"));
  }
}
