use ulua_common::enums::luau_bytecode_type::LuauBytecodeType;

use crate::{
  functions::find_reg_type::find_reg_type, records::bytecode_analysis::BytecodeTypeInfo,
};

pub(crate) fn refine_reg_type(info: &mut BytecodeTypeInfo, reg: u8, pc: i32, ty: u8) {
  if ty != LBC_TYPE_ANY {
    if let Some(reg_type) = find_reg_type(info, reg, pc) {
      if reg_type.r#type == LBC_TYPE_ANY {
        reg_type.r#type = ty;
      }
    } else if (reg as usize) < info.argument_types.len()
      && info.argument_types[reg as usize] == LBC_TYPE_ANY
    {
      info.argument_types[reg as usize] = ty;
    }
  }
}

const LBC_TYPE_ANY: u8 = LuauBytecodeType::LBC_TYPE_ANY.0 as u8;

pub(crate) fn refine_upvalue_type(info: &mut BytecodeTypeInfo, up: i32, ty: u8) {
  if ty != LBC_TYPE_ANY
    && (up as usize) < info.upvalue_types.len()
    && info.upvalue_types[up as usize] == LBC_TYPE_ANY
  {
    info.upvalue_types[up as usize] = ty;
  }
}

#[inline]
pub fn is_custom_userdata_bytecode_type(ty: u8) -> bool {
  ty >= LuauBytecodeType::LBC_TYPE_TAGGED_USERDATA_BASE.0 as u8
    && ty < LuauBytecodeType::LBC_TYPE_TAGGED_USERDATA_END.0 as u8
}

#[inline]
pub fn is_userdata_bytecode_type(ty: u8) -> bool {
  ty as u16 == LuauBytecodeType::LBC_TYPE_USERDATA.0 || is_custom_userdata_bytecode_type(ty)
}

#[inline]
pub fn is_expected_or_unknown_bytecode_type(ty: u8, expected: LuauBytecodeType) -> bool {
  let ty_u16 = ty as u16;
  ty_u16 == LuauBytecodeType::LBC_TYPE_ANY.0 || ty_u16 == expected.0
}

pub fn has_typed_parameters(type_info: &BytecodeTypeInfo) -> bool {
  for &el in &type_info.argument_types {
    if el != LBC_TYPE_ANY {
      return true;
    }
  }
  false
}
