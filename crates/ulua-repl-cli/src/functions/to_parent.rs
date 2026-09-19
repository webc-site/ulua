//! cpp `ReplRequirer.cpp` 的 `static to_parent`。

use core::ffi::c_void;

use ulua_cli_lib::functions::convert_navigation_status::convert_navigation_status;
use ulua_require::enums::luarequire_navigate_result::LuarequireNavigateResult;

use crate::records::repl_requirer::ReplRequirer;

/// # Safety
///
/// `ctx` 必须指向有效的 `ReplRequirer`。
pub unsafe extern "C-unwind" fn to_parent(
  _l: *mut c_void,
  ctx: *mut c_void,
) -> LuarequireNavigateResult {
  let req = unsafe { &mut *(ctx as *mut ReplRequirer) };

  convert_navigation_status(req.vfs.to_parent())
}
