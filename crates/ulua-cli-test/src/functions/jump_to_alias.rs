use core::ffi::{CStr, c_char, c_void};

use ulua_cli_lib::functions::is_absolute_path::is_absolute_path;

use crate::{
  functions::convert_repl_requirer::{convert_navigation_status, luarequire_NavigateResult},
  records::repl_requirer::ReplRequirer,
};

pub unsafe extern "C-unwind" fn jump_to_alias(
  _l: *mut c_void,
  ctx: *mut c_void,
  path: *const c_char,
) -> luarequire_NavigateResult {
  if ctx.is_null() || path.is_null() {
    return luarequire_NavigateResult::NAVIGATE_NOT_FOUND;
  }
  let req = ctx as *mut ReplRequirer;
  let path_str = unsafe { CStr::from_ptr(path).to_string_lossy() };

  if !is_absolute_path(&path_str) {
    return luarequire_NavigateResult::NAVIGATE_NOT_FOUND;
  }

  let status = unsafe { (*req).vfs.reset_to_path(&path_str) };
  convert_navigation_status(status)
}
