//! cpp `ReplRequirer.cpp` 的 `static get_loadname`：`vfs.getAbsoluteFilePath()`。

use core::ffi::{c_char, c_void};

use ulua_require::enums::luarequire_write_result::LuarequireWriteResult;

use crate::{functions::write::write, records::repl_requirer::ReplRequirer};

/// # Safety
///
/// `ctx` 必须指向有效的 `ReplRequirer`；缓冲区参数遵循 `ulua-require` 协议。
pub unsafe extern "C-unwind" fn get_loadname(
  _l: *mut c_void,
  ctx: *mut c_void,
  buffer: *mut c_char,
  buffer_size: usize,
  size_out: *mut usize,
) -> LuarequireWriteResult {
  let req = unsafe { &*(ctx as *const ReplRequirer) };
  let loadname = req.vfs.get_absolute_file_path();

  unsafe { write(Some(loadname.as_str()), buffer, buffer_size, size_out) }
}
