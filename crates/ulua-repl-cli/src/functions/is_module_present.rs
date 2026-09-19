//! cpp `ReplRequirer.cpp` 的 `static is_module_present`。

use core::ffi::c_void;

use ulua_cli_lib::functions::is_file::is_file;

use crate::records::repl_requirer::ReplRequirer;

/// # Safety
///
/// `ctx` 必须指向有效的 `ReplRequirer`。
pub unsafe extern "C-unwind" fn is_module_present(_l: *mut c_void, ctx: *mut c_void) -> bool {
  let req = unsafe { &*(ctx as *const ReplRequirer) };
  is_file(&req.vfs.get_file_path())
}
