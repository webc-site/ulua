//! cpp `ReplRequirer.cpp` 的 `static to_child`。

use core::ffi::{CStr, c_char, c_void};

use ulua_cli_lib::functions::convert_navigation_status::convert_navigation_status;
use ulua_require::enums::luarequire_navigate_result::LuarequireNavigateResult;

use crate::records::repl_requirer::ReplRequirer;

/// # Safety
///
/// `ctx` 必须指向有效的 `ReplRequirer`；`name` 必须是有效 NUL 结尾串。
pub unsafe extern "C-unwind" fn to_child(
  _l: *mut c_void,
  ctx: *mut c_void,
  name: *const c_char,
) -> LuarequireNavigateResult {
  let req = unsafe { &mut *(ctx as *mut ReplRequirer) };
  let name = unsafe { CStr::from_ptr(name) }.to_string_lossy();

  convert_navigation_status(req.vfs.to_child(&name))
}
