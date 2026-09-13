use crate::{
  functions::{lua_gettop::lua_gettop, lua_l_checkunsigned::lua_l_checkunsigned},
  macros::trim::trim,
  type_aliases::{b_uint::BUint, lua_state::lua_State},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn andaux(l: *mut lua_State) -> BUint {
  unsafe {
    let n = lua_gettop(l);
    let mut r: BUint = !(0 as BUint);

    for i in 1..=n {
      r &= lua_l_checkunsigned(l, i);
    }

    trim(r)
  }
}
