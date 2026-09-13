use core::ffi::c_char;

use crate::{
  functions::set_compile_constant_string::set_compile_constant_string,
  type_aliases::lua_compile_constant::LuaCompileConstant,
};

pub fn luau_set_compile_constant_string(constant: LuaCompileConstant, s: *const c_char, l: usize) {
  set_compile_constant_string(constant, s, l);
}
