use crate::{
  functions::set_compile_constant_boolean::set_compile_constant_boolean,
  type_aliases::lua_compile_constant::LuaCompileConstant,
};

pub fn luau_set_compile_constant_boolean(constant: LuaCompileConstant, b: bool) {
  set_compile_constant_boolean(constant, b);
}
