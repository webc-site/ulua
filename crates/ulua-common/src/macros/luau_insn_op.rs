use crate::records::instruction::Instruction;

#[inline(always)]
pub const fn luau_insn_op(insn: u32) -> u32 {
  Instruction(insn).op() as u32
}
