use core::ffi::{CStr, c_char, c_void};

use ulua_cli_lib::methods::vfs_navigator_reset_to_std_in::vfs_navigator_reset_to_std_in;
use ulua_vm::type_aliases::lua_state::lua_State;

use crate::{
  functions::convert_repl_requirer::{convert, luarequire_NavigateResult},
  records::repl_requirer::ReplRequirer,
};

pub unsafe fn reset(
  _l: *mut lua_State,
  ctx: *mut c_void,
  requirer_chunkname: *const c_char,
) -> luarequire_NavigateResult {
  unsafe {
    let req = &mut *(ctx as *mut ReplRequirer);

    let chunkname = CStr::from_ptr(requirer_chunkname).to_string_lossy();
    if chunkname == "=stdin" {
      convert(vfs_navigator_reset_to_std_in(&mut req.vfs))
    } else if !chunkname.is_empty() && chunkname.as_bytes()[0] == b'@' {
      convert(req.vfs.reset_to_path(&chunkname[1..]))
    } else {
      luarequire_NavigateResult::NAVIGATE_NOT_FOUND
    }
  }
}
