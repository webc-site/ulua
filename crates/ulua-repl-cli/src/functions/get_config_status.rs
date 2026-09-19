//! cpp `ReplRequirer.cpp` 的 `static get_config_status`。

use core::ffi::c_void;

use ulua_cli_lib::functions::convert_config_status::convert_config_status;
use ulua_require::enums::luarequire_config_status::LuarequireConfigStatus;

use crate::records::repl_requirer::ReplRequirer;

/// # Safety
///
/// `ctx` 必须指向有效的 `ReplRequirer`。
pub unsafe extern "C-unwind" fn get_config_status(
  _l: *mut c_void,
  ctx: *mut c_void,
) -> LuarequireConfigStatus {
  let req = unsafe { &*(ctx as *const ReplRequirer) };
  convert_config_status(req.vfs.get_config_status())
}
