use crate::{
  functions::set_compile_constant_nil::set_compile_constant_nil,
  type_aliases::lua_compile_constant::LuaCompileConstant,
};

pub fn luau_set_compile_constant_nil(constant: LuaCompileConstant) {
  set_compile_constant_nil(constant);
}
