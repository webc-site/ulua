//! cpp `ReplRequirer.cpp` 的 `static get_cache_key`：与 loadname 同为绝对路径。

use core::ffi::{c_char, c_void};

use ulua_require::enums::luarequire_write_result::LuarequireWriteResult;

use crate::{functions::write::write, records::repl_requirer::ReplRequirer};

/// # Safety
///
/// `ctx` 必须指向有效的 `ReplRequirer`；缓冲区参数遵循 `ulua-require` 协议。
pub unsafe extern "C-unwind" fn get_cache_key(
  _l: *mut c_void,
  ctx: *mut c_void,
  buffer: *mut c_char,
  buffer_size: usize,
  size_out: *mut usize,
) -> LuarequireWriteResult {
  let req = unsafe { &*(ctx as *const ReplRequirer) };
  let cache_key = req.vfs.get_absolute_file_path();

  unsafe { write(Some(cache_key.as_str()), buffer, buffer_size, size_out) }
}
