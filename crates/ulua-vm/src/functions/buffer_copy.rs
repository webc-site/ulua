use core::{
  ffi::{c_char, c_int},
  ptr::copy,
};

use crate::{
  functions::{
    buffer_errors::buffer_oob_error, lua_l_checkbuffer::lua_l_checkbuffer,
    lua_l_checkinteger::lua_l_checkinteger, lua_l_optinteger::lua_l_optinteger,
  },
  macros::isoutofbounds::isoutofbounds,
  type_aliases::lua_state::lua_State,
};

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub(crate) unsafe extern "C-unwind" fn buffer_copy(l: *mut lua_State) -> c_int {
  unsafe {
    let mut tlen: usize = 0;
    let tbuf = lua_l_checkbuffer(l, 1, &mut tlen);
    let toffset = lua_l_checkinteger(l, 2);

    let mut slen: usize = 0;
    let sbuf = lua_l_checkbuffer(l, 3, &mut slen);
    let soffset = lua_l_optinteger(l, 4, 0);

    // C++ evaluates `int(slen) - soffset` as the default eagerly (signed overflow
    // is UB upstream for soffset = INT_MIN); wrapping_sub reproduces the two's-
    // complement value C++ relies on, which the `size < 0` / isoutofbounds checks
    // below then reject. (Upstream UBSan: lbuflib.cpp:257.)
    let size = lua_l_optinteger(l, 5, (slen as c_int).wrapping_sub(soffset));

    if size < 0 {
      buffer_oob_error(l);
    }

    if isoutofbounds(soffset, slen, size as usize) {
      buffer_oob_error(l);
    }

    if isoutofbounds(toffset, tlen, size as usize) {
      buffer_oob_error(l);
    }

    copy(
      (sbuf as *const c_char).offset(soffset as isize),
      (tbuf as *mut c_char).offset(toffset as isize),
      size as usize,
    );

    0
  }
}
