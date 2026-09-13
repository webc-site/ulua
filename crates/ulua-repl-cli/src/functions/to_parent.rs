use core::ffi::c_void;

use ulua_vm::type_aliases::lua_state::lua_State;

use crate::{
  functions::convert_repl_requirer::{convert, luarequire_NavigateResult},
  records::repl_requirer::ReplRequirer,
};

/// # Safety
///
/// `ctx` must be a non-null, properly aligned pointer to a valid `ReplRequirer`.
pub unsafe fn to_parent(_l: *mut lua_State, ctx: *mut c_void) -> luarequire_NavigateResult {
  unsafe {
    let req = &mut *(ctx as *mut ReplRequirer);
    convert(req.vfs.to_parent())
  }
}
