use core::{
  ffi::{CStr, c_char, c_void},
  str::from_utf8,
};

use ulua_cli_lib::methods::vfs_navigator_reset_to_std_in::vfs_navigator_reset_to_std_in;

use crate::{
  functions::convert_repl_requirer::{convert_navigation_status, luarequire_NavigateResult},
  records::repl_requirer::ReplRequirer,
};

pub unsafe extern "C-unwind" fn reset(
  _l: *mut c_void,
  ctx: *mut c_void,
  requirer_chunkname: *const c_char,
) -> luarequire_NavigateResult {
  if ctx.is_null() || requirer_chunkname.is_null() {
    return luarequire_NavigateResult::NAVIGATE_NOT_FOUND;
  }
  unsafe {
    let req = ctx as *mut ReplRequirer;
    let chunkname = CStr::from_ptr(requirer_chunkname).to_bytes();

    if chunkname == b"=stdin" {
      convert_navigation_status(vfs_navigator_reset_to_std_in(&mut (*req).vfs))
    } else if let Some(stripped) = chunkname.strip_prefix(b"@") {
      let path = from_utf8(stripped).unwrap_or("");
      convert_navigation_status((*req).vfs.reset_to_path(path))
    } else {
      luarequire_NavigateResult::NAVIGATE_NOT_FOUND
    }
  }
}
