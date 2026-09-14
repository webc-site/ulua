use core::{ffi::c_char, fmt::Arguments};

use crate::{functions::lua_o_pushvfstring::luaO_pushvfstring, type_aliases::lua_state::lua_State};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn lua_o_pushfstring(
  l: *mut lua_State,
  fmt: *const c_char,
  args: Arguments<'_>,
) -> *const c_char {
  unsafe {
    // In the Luau Rust port, printf-style varargs are handled by passing core::fmt::Arguments.
    luaO_pushvfstring(l, fmt, args)
  }
}

pub use lua_o_pushfstring as luaO_pushfstring;
