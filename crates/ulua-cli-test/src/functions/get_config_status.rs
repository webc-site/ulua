use core::ffi::c_void;

use ulua_cli_lib::functions::convert_config_status::convert_config_status as convert;
use ulua_require::enums::luarequire_config_status::LuarequireConfigStatus as luarequire_ConfigStatus;

use crate::records::repl_requirer::ReplRequirer;

pub unsafe extern "C-unwind" fn get_config_status(
  _l: *mut c_void,
  ctx: *mut c_void,
) -> luarequire_ConfigStatus {
  let req = ctx as *mut ReplRequirer;
  let status = unsafe { (*req).vfs.get_config_status() };
  convert(status)
}
