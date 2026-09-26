use core::ffi::c_void;

use crate::type_aliases::lua_c_function::LuaCFunction;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub(crate) struct CCallS {
  pub(crate) func: LuaCFunction,
  pub(crate) ud: *mut c_void,
}
