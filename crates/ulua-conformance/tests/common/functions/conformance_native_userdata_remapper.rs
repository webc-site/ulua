use core::{
  ffi::{c_char, c_void},
  slice::from_raw_parts,
  str::from_utf8,
};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn conformance_native_userdata_remapper(
  _context: *mut c_void,
  name: *const c_char,
  name_length: usize,
) -> u8 {
  if name.is_null() || name_length == 0 {
    return 0xff;
  }
  let name_bytes = unsafe { from_raw_parts(name as *const u8, name_length) };
  let name_str = from_utf8(name_bytes).unwrap_or("");
  match name_str {
    "extra" => 0,
    "color" => 1,
    "vec2" => 2,
    "mat3" => 3,
    "vertex" => 4,
    _ => 0xff,
  }
}
