use core::ffi::c_void;

use crate::{
  functions::convert_repl_requirer::{convert_navigation_status, luarequire_NavigateResult},
  records::repl_requirer::ReplRequirer,
};

pub unsafe extern "C-unwind" fn to_parent(
  _l: *mut c_void,
  ctx: *mut c_void,
) -> luarequire_NavigateResult {
  let req = ctx as *mut ReplRequirer;
  let status = unsafe { (*req).vfs.to_parent() };
  convert_navigation_status(status)
}
