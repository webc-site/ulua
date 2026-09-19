use ulua_common::enums::luau_bytecode_type;

use crate::records::bytecode_type_info::BytecodeTypeInfo;
const LBC_TYPE_ANY: u8 = luau_bytecode_type::LBC_TYPE_ANY.0 as u8;

pub(crate) fn refine_upvalue_type(info: &mut BytecodeTypeInfo, up: i32, ty: u8) {
  if ty != LBC_TYPE_ANY
    && (up as usize) < info.upvalue_types.len()
    && info.upvalue_types[up as usize] == LBC_TYPE_ANY
  {
    info.upvalue_types[up as usize] = ty;
  }
}
