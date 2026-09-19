//! cpp `ReplRequirer.cpp` 的 `static jump_to_alias`：仅接受绝对路径。

use core::ffi::{CStr, c_char, c_void};

use ulua_cli_lib::functions::{
  convert_navigation_status::convert_navigation_status, is_absolute_path::is_absolute_path,
};
use ulua_require::enums::luarequire_navigate_result::LuarequireNavigateResult;

use crate::records::repl_requirer::ReplRequirer;

/// # Safety
///
/// `ctx` 必须指向有效的 `ReplRequirer`；`path` 必须是有效 NUL 结尾串。
pub unsafe extern "C-unwind" fn jump_to_alias(
  _l: *mut c_void,
  ctx: *mut c_void,
  path: *const c_char,
) -> LuarequireNavigateResult {
  let req = unsafe { &mut *(ctx as *mut ReplRequirer) };
  let path = unsafe { CStr::from_ptr(path) }.to_string_lossy();

  if !is_absolute_path(&path) {
    return LuarequireNavigateResult::NAVIGATE_NOT_FOUND;
  }

  convert_navigation_status(req.vfs.reset_to_path(&path))
}
