use core::ffi::c_void;

use crate::{
  functions::convert_repl_requirer_alt_b::{
    convert_vfs_navigator_config_status, luarequire_ConfigStatus,
  },
  records::repl_requirer::ReplRequirer,
};

pub unsafe extern "C-unwind" fn get_config_status(
  _l: *mut c_void,
  ctx: *mut c_void,
) -> luarequire_ConfigStatus {
  let req = ctx as *mut ReplRequirer;
  let status = unsafe { (*req).vfs.get_config_status() };
  convert_vfs_navigator_config_status(status)
}
