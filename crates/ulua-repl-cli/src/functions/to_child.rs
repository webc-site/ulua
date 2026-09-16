use core::ffi::{CStr, c_char, c_void};

use ulua_vm::type_aliases::lua_state::lua_State;

use crate::{
  functions::convert_repl_requirer::{convert, luarequire_NavigateResult},
  records::repl_requirer::ReplRequirer,
};

/// # Safety
///
/// `ctx` must be a non-null, properly aligned pointer to a valid `ReplRequirer`.
/// `name` must be a valid, null-terminated C string.
pub unsafe fn to_child(
  _l: *mut lua_State,
  ctx: *mut c_void,
  name: *const c_char,
) -> luarequire_NavigateResult {
  unsafe {
    let req = &mut *(ctx as *mut ReplRequirer);
    let name = CStr::from_ptr(name).to_string_lossy();
    convert(req.vfs.to_child(&name))
  }
}
