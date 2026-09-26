use crate::{
  enums::size_x_64::SizeX64,
  records::{operand_x_64::OperandX64, register_x_64::RegisterX64},
};

// kOffsetOfTKeyTagNext = 12（来自 EmitCommon.h）
// offsetof(LuaNode, key) = 16（LuaNode 占 32 字节，前 16 字节是 val，之后是 key）
// 因此偏移为 16 + 12 = 28
// 使用 dword[base + disp] 形式
pub const fn luau_node_key_tag(node: RegisterX64) -> OperandX64 {
  OperandX64::mem(SizeX64::Dword, RegisterX64::NOREG, 0, node, 28)
}
