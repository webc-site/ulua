use core::{ffi::c_char, slice::from_raw_parts};

use ulua_vm::records::lua_state::lua_State;
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_api_atoms_useratom(
  _l: *mut lua_State,
  s: *const c_char,
  len: usize,
) -> i16 {
  if s.is_null() || len == 0 {
    return -1;
  }
  let bytes = unsafe { from_raw_parts(s as *const u8, len) };
  match bytes {
    b"string" => 0,
    b"important" => 1,
    _ => -1,
  }
}
