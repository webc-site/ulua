use core::ffi::{c_char, c_void};

use ulua_require::enums::luarequire_write_result::luarequire_WriteResult;

use crate::{functions::write::write, records::repl_requirer::ReplRequirer};

pub unsafe extern "C-unwind" fn get_config(
  _l: *mut c_void,
  ctx: *mut c_void,
  buffer: *mut c_char,
  buffer_size: usize,
  size_out: *mut usize,
) -> luarequire_WriteResult {
  let req = ctx as *mut ReplRequirer;
  let config = unsafe { (*req).vfs.get_config() };
  unsafe { write(config.as_deref(), buffer, buffer_size, size_out) }
}
