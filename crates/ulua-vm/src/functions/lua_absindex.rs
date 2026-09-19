use core::ffi::c_int;

use crate::{
  macros::{api_check::api_check, cast_int::cast_int, lua_ispseudo::lua_ispseudo},
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn lua_absindex(l: *mut lua_State, idx: c_int) -> c_int {
  let top_minus_base = unsafe { (*l).top.offset_from((*l).base) };

  api_check!(
    l,
    (idx > 0 && idx <= cast_int!(top_minus_base))
      || (idx < 0 && -idx <= cast_int!(top_minus_base))
      || lua_ispseudo(idx)
  );

  if idx > 0 || lua_ispseudo(idx) {
    idx
  } else {
    cast_int!(unsafe { (*l).top.offset_from((*l).base) }) + idx + 1
  }
}
