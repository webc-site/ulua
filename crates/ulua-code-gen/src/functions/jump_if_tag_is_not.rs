use ulua_vm::enums::lua_type::LuaType;

use crate::{
  enums::condition_x_64::ConditionX64,
  functions::luau_reg_tag::luau_reg_tag,
  records::{assembly_builder_x_64::AssemblyBuilderX64, label::Label},
};

pub fn jump_if_tag_is_not(
  build: &mut AssemblyBuilderX64,
  ri: i32,
  tag: LuaType,
  label: &mut Label,
) {
  build.cmp(luau_reg_tag(ri), (tag as i32).into());
  build.jcc(ConditionX64::NotEqual, label);
}
