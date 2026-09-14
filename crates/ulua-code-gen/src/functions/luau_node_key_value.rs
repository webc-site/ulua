use crate::{
  enums::size_x_64::SizeX64,
  records::{operand_x_64::OperandX64, register_x_64::RegisterX64},
};

#[inline]
pub fn luau_node_key_value(node: RegisterX64) -> OperandX64 {
  // offsetof(LuaNode, key) = 16 (LuaNode is 32 bytes, first 16 bytes are base, then key)
  // offsetof(TKey, value) = 0 (TKey starts with Value value)
  // So the offset is 16 + 0 = 16
  // We use qword[base + disp] form
  OperandX64::mem(SizeX64::Qword, RegisterX64::NOREG, 0, node, 16)
}
