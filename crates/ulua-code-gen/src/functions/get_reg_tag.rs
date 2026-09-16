use ulua_common::{FFlag::LuauCodegenRegTag2, enums::luau_bytecode_type::LBC_TYPE_ANY};

use crate::{
  functions::find_reg_type::find_reg_type, records::bytecode_type_info::BytecodeTypeInfo,
};

pub fn get_reg_tag(
  reg_tags: &mut [u8; 256],
  bc_type_info: &mut BytecodeTypeInfo,
  reg: u8,
  pc: i32,
) -> u8 {
  if !LuauCodegenRegTag2.get() {
    return reg_tags[reg as usize];
  }

  if let Some(type_info) = find_reg_type(bc_type_info, reg, pc)
    && type_info.r#type != LBC_TYPE_ANY.0 as u8
  {
    let ty = type_info.r#type;
    reg_tags[reg as usize] = ty;
    return ty;
  }

  reg_tags[reg as usize]
}
