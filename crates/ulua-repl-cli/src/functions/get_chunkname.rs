use alloc::string::String;
use core::ffi::{c_char, c_void};

use ulua_code_gen::type_aliases::lua_state::lua_State;

use crate::{
  functions::write::{luarequire_WriteResult, write},
  records::repl_requirer::ReplRequirer,
};

pub unsafe fn get_chunkname(
  _l: *mut lua_State,
  ctx: *mut c_void,
  buffer: *mut c_char,
  buffer_size: usize,
  size_out: *mut usize,
) -> luarequire_WriteResult {
  unsafe {
    let req = &*(ctx as *const ReplRequirer);
    let path = req.vfs.get_file_path();
    let chunkname = alloc::format!("@{}", path);

    write(
      &chunkname as *const String as *const c_void,
      buffer,
      buffer_size,
      size_out,
    )
  }
}
