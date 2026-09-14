use crate::{
  functions::set_compile_constant_vector::set_compile_constant_vector,
  type_aliases::lua_compile_constant::LuaCompileConstant,
};

pub fn luau_set_compile_constant_vector(
  constant: LuaCompileConstant,
  x: f32,
  y: f32,
  z: f32,
  w: f32,
) {
  set_compile_constant_vector(constant as _, x, y, z, w);
}
