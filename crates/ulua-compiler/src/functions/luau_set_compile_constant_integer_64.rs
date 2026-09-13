use crate::{
  functions::set_compile_constant_integer_64::set_compile_constant_integer_64,
  type_aliases::{compile_constant::CompileConstant, lua_compile_constant::LuaCompileConstant},
};

pub fn luau_set_compile_constant_integer_64(constant: *mut LuaCompileConstant, l: i64) {
  set_compile_constant_integer_64(constant as CompileConstant, l);
}
