use core::ffi::{c_char, c_void};

use ulua_code_gen::type_aliases::lua_state::lua_State;

use crate::{
  functions::write::{luarequire_WriteResult, write},
  records::repl_requirer::ReplRequirer,
};

pub unsafe fn get_config(
  _l: *mut lua_State,
  ctx: *mut c_void,
  buffer: *mut c_char,
  buffer_size: usize,
  size_out: *mut usize,
) -> luarequire_WriteResult {
  unsafe {
    let req = &*(ctx as *const ReplRequirer);
    let config = req.vfs.get_config();
    match config {
      Some(ref s) => write(s.as_ptr() as *const c_void, buffer, buffer_size, size_out),
      None => luarequire_WriteResult::WRITE_FAILURE,
    }
  }
}
