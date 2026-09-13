use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::condition_x_64::ConditionX64,
  functions::{
    jump_if_tag_is::jump_if_tag_is, jump_if_tag_is_not::jump_if_tag_is_not,
    luau_reg_value_int::luau_reg_value_int,
  },
  records::{assembly_builder_x_64::AssemblyBuilderX64, label::Label},
};

pub fn jump_if_truthy(
  build: &mut AssemblyBuilderX64,
  ri: i32,
  target: &mut Label,
  fallthrough: &mut Label,
) {
  jump_if_tag_is(build, ri, LuaType::Nil, fallthrough); // false if nil
  jump_if_tag_is_not(build, ri, LuaType::Boolean, target); // true if not nil or boolean

  build.cmp(luau_reg_value_int(ri), 0.into());
  build.jcc(ConditionX64::NotEqual, target); // true if boolean value is 'true'
}
