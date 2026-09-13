use core::ffi::c_int;

use crate::{
  functions::{lua_l_checkunsigned::lua_l_checkunsigned, lua_pushunsigned::lua_pushunsigned},
  type_aliases::{b_uint::BUint, lua_state::lua_State},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe extern "C-unwind" fn b_countrz(l: *mut lua_State) -> c_int {
  unsafe {
    let v = lua_l_checkunsigned(l, 1) as BUint;

    // trailing_zeros 与 C++ __builtin_ctz 等价（v == 0 时返回 32），单指令实现
    let r = v.trailing_zeros();

    lua_pushunsigned(l, r);
    1
  }
}
