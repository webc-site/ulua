use core::ffi::c_int;

use crate::records::lua_table::LuaTable;
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct CallContext {
  pub(crate) t: *mut LuaTable,
  pub(crate) nhsize: c_int,
}
