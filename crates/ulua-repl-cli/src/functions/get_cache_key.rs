use core::ffi::{c_char, c_void};

use ulua_vm::type_aliases::lua_state::lua_State;

use crate::{
  functions::write::{luarequire_WriteResult, write},
  records::repl_requirer::ReplRequirer,
};

pub unsafe fn get_cache_key(
  _l: *mut lua_State,
  ctx: *mut c_void,
  buffer: *mut c_char,
  buffer_size: usize,
  size_out: *mut usize,
) -> luarequire_WriteResult {
  unsafe {
    let req = &*(ctx as *const ReplRequirer);
    // cpp get_cache_key 即单行 write(getAbsoluteFilePath), 缓冲逻辑复用 write()
    write(
      &req.vfs.get_absolute_file_path() as *const String as *const c_void,
      buffer,
      buffer_size,
      size_out,
    )
  }
}
