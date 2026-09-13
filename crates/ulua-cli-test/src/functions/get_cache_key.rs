use core::ffi::{c_char, c_void};

use ulua_require::enums::luarequire_write_result::luarequire_WriteResult;

use crate::{functions::write::write, records::repl_requirer::ReplRequirer};

pub unsafe extern "C-unwind" fn get_cache_key(
  _l: *mut c_void,
  ctx: *mut c_void,
  buffer: *mut c_char,
  buffer_size: usize,
  size_out: *mut usize,
) -> luarequire_WriteResult {
  let req = ctx as *mut ReplRequirer;
  let path = unsafe { (*req).vfs.get_absolute_file_path() };
  unsafe { write(Some(&path), buffer, buffer_size, size_out) }
}
