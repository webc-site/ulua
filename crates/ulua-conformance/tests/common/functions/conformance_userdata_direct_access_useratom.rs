use core::{
  ffi::{c_char, c_int},
  slice::from_raw_parts,
  str::from_utf8,
};

use ulua_vm::records::lua_state::lua_State;

use crate::common::functions::get_or_create_atom::get_or_create_atom;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_userdata_direct_access_useratom(
  _l: *mut lua_State,
  s: *const c_char,
  len: usize,
) -> i16 {
  if s.is_null() || len == 0 {
    return -1;
  }
  let bytes = unsafe { from_raw_parts(s as *const u8, len) };
  let name = from_utf8(bytes).unwrap_or("");
  get_or_create_atom(name) as c_int as i16
}
