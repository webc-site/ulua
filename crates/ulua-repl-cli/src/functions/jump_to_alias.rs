use core::ffi::{CStr, c_char, c_void};

use ulua_cli_lib::functions::is_absolute_path::is_absolute_path;
use ulua_vm::type_aliases::lua_state::lua_State;

use crate::{
  functions::convert_repl_requirer::{convert, luarequire_NavigateResult},
  records::repl_requirer::ReplRequirer,
};

pub unsafe fn jump_to_alias(
  _l: *mut lua_State,
  ctx: *mut c_void,
  path: *const c_char,
) -> luarequire_NavigateResult {
  unsafe {
    let req = &mut *(ctx as *mut ReplRequirer);

    let path = CStr::from_ptr(path).to_string_lossy();
    if !is_absolute_path(&path) {
      return luarequire_NavigateResult::NAVIGATE_NOT_FOUND;
    }

    convert(req.vfs.reset_to_path(&path))
  }
}
