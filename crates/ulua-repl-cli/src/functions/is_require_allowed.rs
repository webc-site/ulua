use core::ffi::{CStr, c_char, c_void};

use ulua_code_gen::type_aliases::lua_state::lua_State;

pub unsafe extern "C-unwind" fn is_require_allowed(
  _l: *mut lua_State,
  _ctx: *mut c_void,
  requirer_chunkname: *const c_char,
) -> bool {
  unsafe {
    if requirer_chunkname.is_null() {
      return false;
    }
    let chunkname = CStr::from_ptr(requirer_chunkname).to_bytes();
    chunkname == b"=stdin" || (!chunkname.is_empty() && chunkname[0] == b'@')
  }
}
