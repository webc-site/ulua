use core::ffi::c_void;

use ulua_cli_lib::functions::is_file::is_file;

use crate::records::repl_requirer::ReplRequirer;

pub unsafe extern "C-unwind" fn is_module_present(_l: *mut c_void, ctx: *mut c_void) -> bool {
  let req = ctx as *mut ReplRequirer;
  let path = unsafe { (*req).vfs.get_file_path() };
  is_file(&path)
}
