use core::{
  ffi::{c_char, c_int},
  mem::size_of,
};

use crate::{
  enums::lua_status::LuaStatus,
  functions::lua_d_growstack::lua_d_growstack,
  macros::{lua_s_new::luaS_new, setsvalue::setsvalue},
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe fn resume_error(l: *mut lua_State, msg: *const c_char, narg: c_int) -> c_int {
  unsafe {
    // l->top -= narg;
    (*l).top = (*l).top.sub(narg as usize);

    // setsvalue(l, l->top, luaS_new(l, msg));
    // Note: setsvalue! macro expects a pointer to the TValue.
    // (*l).top is a StkId (which is a *mut TValue).
    setsvalue!(l, (*l).top, luaS_new(l, msg));

    // incr_top(l) expands to: { luaD_checkstack(l, 1); l->top++; }
    // We manually perform the incr_top logic here to match the C++ source.

    // stacklimitreached check (simplified for the error-prone macro environment)
    let stack_last = (*l).stack_last as *mut u8;
    let top = (*l).top as *mut u8;
    let limit_reached = (stack_last as usize).wrapping_sub(top as usize) <= size_of::<TValue>();

    if limit_reached {
      lua_d_growstack(l, 1);
    }

    (*l).top = (*l).top.add(1);

    LuaStatus::ErrRun as c_int
  }
}
