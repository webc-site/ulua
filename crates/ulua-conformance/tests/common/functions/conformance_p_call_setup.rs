use ulua_vm::records::lua_state::LuaState;

use crate::common::functions::{
  conformance_p_call_resume_error::conformance_p_call_resume_error, cxxthrow::cxxthrow,
  safe_api::{pushcclosurek, state_mut},
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_p_call_setup(l: *mut LuaState) {
  // 用 NUL 结尾静态名建 `cxxthrow` 闭包并登记为全局。
  pushcclosurek(l, Some(cxxthrow), Some(b"cxxthrow\0"), 0, None);
  state_mut(l).set_global_str("cxxthrow");

  // 同上——登记 `resumeerror` 闭包（桩为 `extern "C-unwind"`）。
  pushcclosurek(
    l,
    Some(conformance_p_call_resume_error),
    Some(b"resumeerror\0"),
    0,
    None,
  );
  state_mut(l).set_global_str("resumeerror");
}
