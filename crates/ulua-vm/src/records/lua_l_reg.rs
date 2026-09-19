//! Source: `VM/include/lualib.h`

use core::ffi::c_char;

use crate::type_aliases::lua_c_function::LuaCFunction;
#[derive(Debug, Clone, Copy)]
pub struct LuaLReg {
  pub name: *const c_char,
  pub func: LuaCFunction,
}
