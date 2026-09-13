use core::ffi::c_void;

use ulua_cli_lib::functions::is_file::is_file;
use ulua_code_gen::type_aliases::lua_state::lua_State;

use crate::records::repl_requirer::ReplRequirer;

pub unsafe fn is_module_present(_l: *mut lua_State, ctx: *mut c_void) -> bool {
  unsafe {
    let req = &*(ctx as *const ReplRequirer);
    let path = req.vfs.get_file_path();
    is_file(&path)
  }
}
