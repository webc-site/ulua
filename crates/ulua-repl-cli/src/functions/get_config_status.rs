use core::ffi::c_void;

use ulua_cli_lib::functions::convert_config_status::convert_config_status as convert;
use ulua_require::enums::luarequire_config_status::LuarequireConfigStatus as luarequire_ConfigStatus;
use ulua_vm::type_aliases::lua_state::lua_State;

use crate::records::repl_requirer::ReplRequirer;

pub unsafe fn get_config_status(_l: *mut lua_State, ctx: *mut c_void) -> luarequire_ConfigStatus {
  unsafe {
    let req = &*(ctx as *const ReplRequirer);
    convert(req.vfs.get_config_status())
  }
}
