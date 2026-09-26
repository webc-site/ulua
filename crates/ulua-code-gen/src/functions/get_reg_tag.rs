use ulua_common::enums::luau_bytecode_type::LuauBytecodeType;

use crate::{
  functions::find_reg_type::find_reg_type, records::bytecode_type_info::BytecodeTypeInfo,
};

/// cpp BytecodeAnalysis.cpp:758-770 `getRegTag`：优先取静态分析声明的类型，
/// 否则回退到前序指令推导出的类型（移植期开关 LuauCodegenRegTag2 在 cpp 中
/// 已删除，逐 pc 线性扫描 regTypes 的旧路径不复存在）。
pub fn get_reg_tag(
  reg_tags: &mut [u8; 256],
  bc_type_info: &mut BytecodeTypeInfo,
  reg: u8,
  pc: i32,
) -> u8 {
  if let Some(type_info) = find_reg_type(bc_type_info, reg, pc)
    && type_info.r#type != LuauBytecodeType::LBC_TYPE_ANY.0 as u8
  {
    let ty = type_info.r#type;
    reg_tags[reg as usize] = ty;
    return ty;
  }

  reg_tags[reg as usize]
}
