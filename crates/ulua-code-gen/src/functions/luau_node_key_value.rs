use crate::{
  enums::size_x_64::SizeX64,
  records::{operand_x_64::OperandX64, register_x_64::RegisterX64},
};

#[inline]
pub fn luau_node_key_value(node: RegisterX64) -> OperandX64 {
  // offsetof(LuaNode, key) = 16（LuaNode 占 32 字节，前 16 字节是 base，之后是 key）
  // offsetof(TKey, value) = 0（TKey 以 Value value 开头）
  // 因此偏移为 16 + 0 = 16
  // 使用 qword[base + disp] 形式
  OperandX64::mem(SizeX64::Qword, RegisterX64::NOREG, 0, node, 16)
}
