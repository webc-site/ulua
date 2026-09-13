use core::ffi::{CStr, c_char, c_void};

pub unsafe extern "C-unwind" fn is_require_allowed(
  _l: *mut c_void,
  _ctx: *mut c_void,
  requirer_chunkname: *const c_char,
) -> bool {
  let chunkname = unsafe {
    if requirer_chunkname.is_null() {
      return false;
    }
    CStr::from_ptr(requirer_chunkname).to_bytes()
  };

  if chunkname == b"=stdin" {
    true
  } else {
    !chunkname.is_empty() && chunkname[0] == b'@'
  }
}
