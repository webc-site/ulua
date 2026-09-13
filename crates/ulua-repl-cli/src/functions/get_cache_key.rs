use core::{
  ffi::{c_char, c_void},
  slice::from_raw_parts_mut,
};

use ulua_code_gen::type_aliases::lua_state::lua_State;

use crate::{functions::write::luarequire_WriteResult, records::repl_requirer::ReplRequirer};

pub unsafe fn get_cache_key(
  _l: *mut lua_State,
  ctx: *mut c_void,
  buffer: *mut c_char,
  buffer_size: usize,
  size_out: *mut usize,
) -> luarequire_WriteResult {
  unsafe {
    if ctx.is_null() {
      return luarequire_WriteResult::WRITE_FAILURE;
    }

    let req = &*(ctx as *const ReplRequirer);
    let path = req.vfs.get_absolute_file_path();
    let path_cstr = path.as_bytes();
    let null_terminated_size = path_cstr.len() + 1;

    if buffer_size < null_terminated_size {
      if !size_out.is_null() {
        *size_out = null_terminated_size;
      }
      return luarequire_WriteResult::WRITE_BUFFER_TOO_SMALL;
    }

    if !buffer.is_null() {
      let src = path_cstr;
      let dst = from_raw_parts_mut(buffer as *mut u8, null_terminated_size);
      dst[..src.len()].copy_from_slice(src);
      dst[src.len()] = 0;
    }

    if !size_out.is_null() {
      *size_out = null_terminated_size;
    }

    luarequire_WriteResult::WRITE_SUCCESS
  }
}
