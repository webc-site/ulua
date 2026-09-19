//! cpp `ReplRequirer.cpp` 的 `static reset`：按 requirer chunkname 复位导航目录。

use core::ffi::{CStr, c_char, c_void};

use ulua_cli_lib::{
  functions::convert_navigation_status::convert_navigation_status,
  methods::vfs_navigator_reset_to_std_in::vfs_navigator_reset_to_std_in,
};
use ulua_require::enums::luarequire_navigate_result::LuarequireNavigateResult;

use crate::records::repl_requirer::ReplRequirer;

/// # Safety
///
/// `ctx` 必须指向有效的 `ReplRequirer`；`requirer_chunkname` 必须是有效 NUL 结尾串。
pub unsafe extern "C-unwind" fn reset(
  _l: *mut c_void,
  ctx: *mut c_void,
  requirer_chunkname: *const c_char,
) -> LuarequireNavigateResult {
  let req = unsafe { &mut *(ctx as *mut ReplRequirer) };
  // chunkname 可为任意字节；与 cpp 一致地按字符串比较，非法序列 lossy 替换
  let chunkname = unsafe { CStr::from_ptr(requirer_chunkname) }.to_string_lossy();

  if chunkname == "=stdin" {
    convert_navigation_status(vfs_navigator_reset_to_std_in(&mut req.vfs))
  } else if let Some(path) = chunkname.strip_prefix('@') {
    convert_navigation_status(req.vfs.reset_to_path(path))
  } else {
    LuarequireNavigateResult::NAVIGATE_NOT_FOUND
  }
}
